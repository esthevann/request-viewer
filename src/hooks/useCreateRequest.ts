import { useMutation, useQueryClient } from "@tanstack/react-query";
import { CreateRequestArgs } from "../models/RequestRecord";
import RequestService from "../services/RequestService";

export default function useCreateRequest() {
    const client = useQueryClient();
    return useMutation((args: CreateRequestArgs) => RequestService.create_request(args.name, args.address), {
        onSuccess: () => {
          client.invalidateQueries(['requests']);
        }
      });
}