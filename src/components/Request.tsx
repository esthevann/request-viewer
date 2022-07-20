import { Link } from "raviger"

interface RequestProps {
    name: string
    address?: string
}

export default function Request(props: RequestProps) {
    return (
        <div className="flex items-center justify-between gap-6 border p-4">
            <div className="flex flex-col gap-2">
                <h1 className="font-bold text-xl">{props.name}</h1>
                {props.address &&
                    <label htmlFor="" className="flex gap-2">
                        Address:
                        <p className="bg-gray-300 px-1">{props.address}</p>
                    </label>
                }
            </div>
            <div className="flex gap-2">
                <Link href="/request" className="bg-blue-500 text-white px-3 py-2 rounded-full">Load</Link>
                <button className="bg-green-700 text-white px-3 py-2 rounded-full">Edit</button>
                <button className="bg-red-500 text-white px-3 py-2 rounded-full">Remove</button>
            </div>

        </div>
    )
}