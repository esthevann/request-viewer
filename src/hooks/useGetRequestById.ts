import { useQuery } from "@tanstack/react-query";
import { RequestRecord } from "../models/RequestRecord";
import RequestService from "../services/RequestService";

export default function useGetRequestById(requestId: string, F?: (r: RequestRecord) => void) {
    return useQuery(["get_request_by_id", requestId], () => RequestService.get_request_by_id(requestId), {
        onSuccess: (data) => {
            if (F) {
                F(data);
            }
        }
    });
}