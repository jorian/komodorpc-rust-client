use serde::Serialize;

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum JsonRpcVersion {
    #[serde(rename = "1.0")]
    V1,
    #[serde(rename = "2.0")]
    V2,
}

#[derive(Debug, Serialize)]
pub struct RpcRequest<P>
    where
        P: Serialize,
{
    jsonrpc: JsonRpcVersion,
    id: String,
    method: String,
    params: P,
}

impl RpcRequest<()> {
    pub fn new0(method: &str) -> RpcRequest<()> {
        RpcRequest::new(JsonRpcVersion::V1, "777", method, ())
    }

    pub fn new1<A>(method: &str, first: A) -> RpcRequest<Vec<A>>
        where
            A: Serialize,
    {
        RpcRequest::new(JsonRpcVersion::V1, "777", method, vec![first]) // Handles the special case of one parameter. A tuple would be serialized as a single value.
    }

    pub fn new2<A, B>(
        method: &str,
        first: A,
        second: B,
    ) -> RpcRequest<(A, B)>
        where
            A: Serialize,
            B: Serialize,
    {
        RpcRequest::new(JsonRpcVersion::V1, "777", method, (first, second))
    }

    pub fn new3<A, B, C>(
        method: &str,
        first: A,
        second: B,
        third: C,
    ) -> RpcRequest<(A, B, C)>
        where
            A: Serialize,
            B: Serialize,
            C: Serialize,
    {
        RpcRequest::new(JsonRpcVersion::V1, "777", method, (first, second, third))
    }

    pub fn new4<A, B, C, D>(
        method: &str,
        first: A,
        second: B,
        third: C,
        fourth: D,
    ) -> RpcRequest<(A, B, C, D)>
        where
            A: Serialize,
            B: Serialize,
            C: Serialize,
            D: Serialize,
    {
        RpcRequest::new(JsonRpcVersion::V1, "777", method, (first, second, third, fourth))
    }

    pub fn new5<A, B, C, D, E>(
        version: JsonRpcVersion,
        id: &str,
        method: &str,
        first: A,
        second: B,
        third: C,
        fourth: D,
        fifth: E,
    ) -> RpcRequest<(A, B, C, D, E)>
        where
            A: Serialize,
            B: Serialize,
            C: Serialize,
            D: Serialize,
            E: Serialize,
    {
        RpcRequest::new(version, id, method, (first, second, third, fourth, fifth))
    }

    pub fn new6<A, B, C, D, E, F>(
        version: JsonRpcVersion,
        id: &str,
        method: &str,
        first: A,
        second: B,
        third: C,
        fourth: D,
        fifth: E,
        sixth: F,
    ) -> RpcRequest<(A, B, C, D, E, F)>
        where
            A: Serialize,
            B: Serialize,
            C: Serialize,
            D: Serialize,
            E: Serialize,
            F: Serialize,
    {
        RpcRequest::new(
            version,
            id,
            method,
            (first, second, third, fourth, fifth, sixth),
        )
    }

    fn new<P>(version: JsonRpcVersion, id: &str, method: &str, params: P) -> RpcRequest<P>
        where
            P: Serialize,
    {
        RpcRequest {
            jsonrpc: version,
            id: id.to_string(),
            method: method.to_string(),
            params: params,
        }
    }
}