import { Link, usePathParams } from "raviger";
import { useState } from "react";
import JSONPretty from "react-json-pretty";
import Spinner from "../components/Spinner";
import useFetchRequestData from "../hooks/useFetchRequestData";
import useSendRequest from "../hooks/useSendRequest";
import { Response } from "../models/Request";

type Method = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export default function RequestView() {
    const path = usePathParams('/:requestId/request');
    const requestId = path?.requestId || "";
    const { name, address, method } = useFetchRequestData(requestId);
    const requestSender = useSendRequest();

    const [response, setResponse] = useState<Response | null>();

    function sendRequest(e: React.MouseEvent<HTMLButtonElement>) {
        e.preventDefault();
        requestSender.mutate({ address, body: "{}", body_type: "Json", method: method as Method }, {
            onSuccess: (data) => {
                setResponse(data);
            }
        });
    }
    return (
        <div className="flex flex-col mt-3 ">
            <div className="flex flex-col items-center">
                <h1 className="text-3xl font-bold">Request</h1>
                <div className="pb-1"></div>
                <Link href="/"> Go back</Link>
                <div className="pb-3"></div>
                <div >
                    {name === "" && (
                        <div>
                            <Spinner />
                        </div>
                    )}

                    {name !== "" && (
                        <div className="flex gap-6 items-center">
                            <div>
                                <p>Name: <code>{name}</code></p>
                                <p>Address: <code>{address}</code></p>
                                <p>Method: <code>{method}</code></p>
                            </div>
                            <Link href={`/${requestId}/edit`} className="bg-green-700 text-white px-3 py-2 rounded-full hover:bg-green-800">Edit</Link>
                        </div>

                    )}
                </div>
                <div className="pb-6"></div>
                <div className="flex justify-center gap-3">
                    <p>Body:</p>
                    <textarea name="" value={"{ }"} cols={20} rows={60} className="text-black w-[300px] h-[150px] pl-1"></textarea>
                </div>
                <button disabled={name === ""} onClick={sendRequest} className='mt-2 bg-blue-500 text-white px-3 py-2 rounded-full hover:bg-blue-600'>Send request</button>
            </div>

            <div className="flex flex-col mt-8 items-center">
                <h2 className="text-2xl font-bold"> Response </h2>
                {response && <div>Status code: {response.status}</div>}
                {response && (
                    <div className="flex gap-6" >
                        <div className="flex flex-col items-center">
                            <h3>Body</h3>
                            <JSONPretty data={JSON.stringify(response.body).replace(/\\n/g, '\n').replace(/\\/g, '')} />
                        </div>
                        <div className="flex flex-col items-center">
                            <h3>Headers</h3>
                            <JSONPretty data={JSON.stringify(response.headers)} />
                        </div>
                    </div>
                )}


            </div>

        </div>
    )
}