import { Link } from "raviger";

export default function RequestView() {
    return (
        <div className="flex flex-col mt-3 items-center">
            <h1 className="text-3xl font-bold">Request</h1>
            <Link href="/"> Go back</Link>
        </div>
    )
}