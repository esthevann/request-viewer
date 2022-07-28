import { useQuery } from "@tanstack/react-query";
import RequestService from "../services/RequestService";

export default function useRequests() {
    return useQuery(["requests"], RequestService.get_all_requests);
}