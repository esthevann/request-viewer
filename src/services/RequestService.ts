import { invoke } from '@tauri-apps/api';
import type { RequestRecord } from '../models/RequestRecord';


async function get_all_requests(): Promise<RequestRecord[]> {
    try {
        let request: RequestRecord[] = await invoke("list_all_requests");
        return request;
    } catch (error) {
        if (error instanceof Error){
            throw error
        } else {
            throw new Error("unknown error")
        }
    }
}

async function get_request_by_id(id: string): Promise<RequestRecord>{
    try {
        let request: RequestRecord = await invoke("get_record_by_id", { id });
        return request;
    } catch (error) {
        if (error instanceof Error){
            throw error
        } else {
            throw new Error("unknown error")
        }
    }
}

async function create_request(name: string, address: string | null): Promise<RequestRecord> {
    try {
        let request: RequestRecord = await invoke("create_request_record", { args: { name, address } });
        return request;
    } catch (error) {
        if (error instanceof Error){
            throw error
        } else {
            throw new Error("unknown error")
        }
    }
}

async function update_request(id: string, address: string | null, name: string): Promise<RequestRecord> {
    try {
        let request: RequestRecord = await invoke("update_request_record", { id, address, name });
        return request;
    } catch (error) {
        if (error instanceof Error){
            throw error
        } else {
            throw new Error("unknown error")
        }
    }
}

export default {
    get_all_requests,
    get_request_by_id,
    create_request,
    update_request
}