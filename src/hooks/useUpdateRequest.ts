import { useMutation, useQueryClient } from "@tanstack/react-query";
import { UpdateRequestArgs } from "../models/RequestRecord";
import RequestService from "../services/RequestService";

export default function useUpdateRequest() {
    const client = useQueryClient();
    return useMutation((args: UpdateRequestArgs) => RequestService.update_request(args.id, args.address, args.name), {
        onSuccess: (updated_request) => {
          client.invalidateQueries(['requests']);
          client.setQueryData(['get_request_by_id', updated_request.id], updated_request);
        }
      });
}