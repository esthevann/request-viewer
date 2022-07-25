export interface RequestRecord {
     id: string, name: string, address: string | null, 
    }

export interface CreateRequestArgs {
    name: string, address: string | null,
}