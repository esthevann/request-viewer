import { useQuery } from "@tanstack/react-query";
import RequestService from "../services/RequestService";

export default function useGetRequestById(requestId: string) {
    return useQuery(["get_request_by_id", requestId], () => RequestService.get_request_by_id(requestId));
}