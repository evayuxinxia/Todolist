use reqwest::Client;
use serde_json::{json, Value};
use anyhow::Result;

const DEFAULT_API_URL: &str = "https://ark.cn-beijing.volces.com/api/v3/chat/completions";
const DEFAULT_MODEL: &str = "doubao-pro";

pub struct AiClient {
    client: Client,
    api_key: Option<String>,
    api_url: String,
    model: String,
}

impl AiClient {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_url: DEFAULT_API_URL.to_string(),
            model: DEFAULT_MODEL.to_string(),
        }
    }

    pub async fn parse_task_text(&self, user_input: &str) -> Result<Value> {
        let system_prompt = r#"你是任务提取助手，只输出JSON，禁止多余文字、解释、markdown。
输入一段用户待办描述，提取所有任务，按规则输出：
1. content：任务完整内容；
2. deadline：计划完成日期，只识别年月日，格式YYYY-MM-DD，无则为空；
3. priority：根据描述判定紧急/高/中/低；
4. workload：预估耗时1-5数字；
规则：
- 提到今天/明天/3天后自动换算对应日期；
- 带有尽快、马上、今晚标记为紧急；
- 复杂长期工作工作量5，简单小事工作量1；
输出严格遵循：{"taskList": [{}]}"#;

        let req_body = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_input}
            ],
            "temperature": 0.1
        });

        let api_key = self.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("API Key 未配置"))?;

        let resp = self.client
            .post(&self.api_url)
            .bearer_auth(api_key)
            .json(&req_body)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("网络请求失败：{}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let error_text = resp.text().await.unwrap_or_else(|_| "未知错误".to_string());
            return Err(anyhow::anyhow!("API 请求失败: {} - {}", status, error_text));
        }

        let res_json: Value = resp.json().await
            .map_err(|e| anyhow::anyhow!("解析返回失败：{}", e))?;

        let raw_content = res_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("大模型无返回内容"))?;

        let clean_str = raw_content
            .replace("```json", "")
            .replace("```", "")
            .trim();

        let task_data: Value = serde_json::from_str(clean_str)
            .map_err(|e| anyhow::anyhow!("模型输出JSON格式错误：{} - 原始内容: {}", e, clean_str))?;

        Ok(task_data)
    }
}