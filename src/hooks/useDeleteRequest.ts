import { useMutation, useQueryClient } from "@tanstack/react-query";
import RequestService from "../services/RequestService";

export default function useDeleteRequest() {
    const client = useQueryClient();
    return useMutation((id: string) => RequestService.delete_request(id), {
        onSuccess: () => {
          client.invalidateQueries(['requests']);
        }
      });
}