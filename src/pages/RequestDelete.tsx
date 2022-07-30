import { navigate, usePathParams } from "raviger";
import Request from "../components/Request";
import Spinner from "../components/Spinner";
import useDeleteRequest from "../hooks/useDeleteRequest";
import useFetchRequestData from "../hooks/useFetchRequestData";

type Method = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export default function RequestDeletePage(){
    const path = usePathParams('/:requestId/delete');
    const requestId = path?.requestId || "";
    
    const {name, address, method} = useFetchRequestData(requestId);
    const requestDeleter = useDeleteRequest();

    function deleteRequest() {
        requestDeleter.mutate(requestId, {
            onSuccess: () => {
                navigate("/");
            }
        })
    }
    return (
        <div className="h-screen flex flex-col justify-center items-center">
            <div className="flex flex-col border items-center  px-20 py-6">
                <h1 className="text-3xl font-bold">Delete Request</h1>
                <div className="pb-3"></div>
                <p>Are you sure you want to delete this request record?</p>
                {name === "" && (
                    <div className="flex justify-center items-center">
                        <div className="pb-3"></div>
                        <Spinner/>
                    </div>
                )}
                {name !== "" && (
                    <Request id={requestId}  method={method as Method} address={address} name={name}/>
                )}
                <div className="pb-8"></div>
                <div className="flex gap-3">
                    <button className="bg-red-500 text-white px-3 py-2 rounded-full" onClick={deleteRequest}>Delete</button>
                    <button className="bg-blue-500 text-white px-3 py-2 rounded-full" onClick={() => navigate("/")}>Cancel</button>
                </div>
            </div>
        </div>
    )
}