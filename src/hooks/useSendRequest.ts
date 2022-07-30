import { useMutation } from "@tanstack/react-query";
import { NewRequest } from "../models/Request";
import SendService from "../services/SendService";

export default function useSendRequest() {
    return useMutation((args: NewRequest) => SendService.send_request(args));
}