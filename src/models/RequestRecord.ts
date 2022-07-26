export interface RequestRecord {
     id: string, name: string, address: string | null, created_at: string, method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE"
    }

export interface CreateRequestArgs {
    name: string, address: string | null,
}

export interface UpdateRequestArgs {
    id: string,
    name: string,
    address: string | null,
    method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE"
}