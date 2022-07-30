import { invoke } from "@tauri-apps/api";
import type { Response, NewRequest } from "../models/Request"

async function send_request(request: NewRequest): Promise<Response> {
    try {
        const req: Response = await invoke("send_request", { request });
        return req;
    } catch (error) {
        if (error instanceof Error) {
            throw error
        } else {
            throw new Error(`error: ${error}`)
        }
    }
}

export default {
    send_request
}