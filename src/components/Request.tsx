import { Link, navigate } from "raviger"
import Method from "./Method"

export interface RequestProps {
    id: string
    name: string
    address?: string
    method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE"
}

export default function Request(props: RequestProps) {
    return (
        <div className="flex items-center justify-between gap-32 border p-9">
            <div className="flex flex-col gap-3">
                <div className="flex items-center gap-2">
                    <h1 className="font-bold text-xl">{props.name}</h1>
                    <Method method={props.method} />
                </div>
                

                <label htmlFor="" className="flex gap-2">
                    Address:
                    {props.address && (
                        <p className="bg-gray-300 px-1 text-black">{props.address}</p>
                    )}
                </label>

            </div>
            <div className="flex gap-2">
                <Link href={`/${props.id}/request`} className="bg-blue-500 text-white px-3 py-2 rounded-full">Load</Link>
                <Link href={`/${props.id}/edit`} className="bg-green-700 text-white px-3 py-2 rounded-full">Edit</Link>
                <button className="bg-red-500 text-white px-3 py-2 rounded-full" onClick={() => navigate(`/${props.id}/delete`)}>Remove</button>
            </div>
        </div>
    )
}