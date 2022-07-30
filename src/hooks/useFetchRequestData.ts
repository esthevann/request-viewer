import { useState } from "react";
import useGetRequestById from "./useGetRequestById";


export default function useFetchRequestData(id: string) {
    const [name, setName] = useState("");
    const [address, setAdress] = useState("");
    const [method, setMethod] = useState("GET");

    const { isLoading } = useGetRequestById(id, (e) => { 
        setName(e.name);
        if (e.address) {
            setAdress(e.address);
        }
        setMethod(e.method);
    });

    return {
        name,
        address,
        method,
        setName,
        setAdress,
        setMethod,
        isLoading
    }
}