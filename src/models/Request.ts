export interface NewRequest {
    address: string,
    method: "GET" | "POST" | "PUT" | "PATCH",
    body: string,
    body_type: "Json" | "Text" | "Bytes",
}

export interface Response {
    status: number,
    body: string,
    headers: Record<string, string>,
}