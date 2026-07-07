use super::ServerReportError;
use shared_types::StatusAck;

pub fn parse_status_ack(response: &str) -> Result<StatusAck, ServerReportError> {
    parse_json_response(response)
}

pub fn parse_json_response<T>(response: &str) -> Result<T, ServerReportError>
where
    T: serde::de::DeserializeOwned,
{
    let (head, body) = split_http_response(response)?;
    let status_line = head
        .lines()
        .next()
        .ok_or_else(|| ServerReportError::InvalidResponse("HTTP 响应缺少状态行".to_string()))?;
    let status = parse_status_code(status_line)?;

    if !(200..300).contains(&status) {
        return Err(ServerReportError::ServerRejected {
            status,
            body: body.to_string(),
        });
    }

    let body = decode_body(head, body)?;
    serde_json::from_str(&body)
        .map_err(|error| ServerReportError::InvalidResponse(error.to_string()))
}

fn split_http_response(response: &str) -> Result<(&str, &str), ServerReportError> {
    if let Some(parts) = response.split_once("\r\n\r\n") {
        return Ok(parts);
    }

    response.split_once("\n\n").ok_or_else(|| {
        let preview: String = response.chars().take(120).collect();
        ServerReportError::InvalidResponse(format!(
            "HTTP 响应缺少 body，长度={}，预览={preview:?}",
            response.len()
        ))
    })
}

fn decode_body(head: &str, body: &str) -> Result<String, ServerReportError> {
    let is_chunked = head.lines().any(|line| {
        line.to_ascii_lowercase()
            .contains("transfer-encoding: chunked")
    });

    if is_chunked {
        decode_chunked_body(body)
    } else {
        Ok(body.to_string())
    }
}

fn decode_chunked_body(body: &str) -> Result<String, ServerReportError> {
    let mut decoded = String::new();
    let mut rest = body;

    loop {
        let Some((size_text, after_size)) = rest.split_once("\r\n") else {
            return Err(ServerReportError::InvalidResponse(
                "chunked 响应缺少 chunk size".to_string(),
            ));
        };
        let size = usize::from_str_radix(size_text.trim(), 16).map_err(|_| {
            ServerReportError::InvalidResponse("chunked 响应 size 不是十六进制".to_string())
        })?;

        if size == 0 {
            break;
        }

        if after_size.len() < size + 2 {
            return Err(ServerReportError::InvalidResponse(
                "chunked 响应内容长度不足".to_string(),
            ));
        }

        decoded.push_str(&after_size[..size]);
        rest = &after_size[size + 2..];
    }

    Ok(decoded)
}

fn parse_status_code(status_line: &str) -> Result<u16, ServerReportError> {
    let mut parts = status_line.split_whitespace();
    let _http_version = parts
        .next()
        .ok_or_else(|| ServerReportError::InvalidResponse("状态行缺少协议版本".to_string()))?;
    let status_text = parts
        .next()
        .ok_or_else(|| ServerReportError::InvalidResponse("状态行缺少状态码".to_string()))?;
    status_text
        .parse()
        .map_err(|_| ServerReportError::InvalidResponse("状态码不是数字".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_status_ack_accepts_chunked_response() {
        let body = r#"{"accepted":true,"client_id":"client-a","message_id":"msg-1"}"#;
        let response = format!(
            "HTTP/1.1 200 OK\r\ntransfer-encoding: chunked\r\n\r\n{:x}\r\n{}\r\n0\r\n\r\n",
            body.len(),
            body
        );

        let ack = parse_status_ack(&response).expect("chunked response must parse");

        assert!(ack.accepted);
        assert_eq!(ack.client_id, "client-a");
        assert_eq!(ack.message_id, "msg-1");
    }
}
