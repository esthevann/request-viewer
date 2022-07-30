interface MethodProps {
    method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE"
}

export default function Method(props: MethodProps) {
    if (props.method === "GET"){
        return (
            <div className="bg-green-500 px-2  rounded-md">
                {props.method}
            </div>
        )
    } else if (props.method === "POST") {
        return (
            <div className="bg-purple-600 px-2  rounded-md">
                {props.method}
            </div>
        )
    } else if (props.method === "PUT") {
        return (
            <div className="bg-orange-500 px-2  rounded-md">
                {props.method}
            </div>
        )
    } else if (props.method === "PATCH") {
        return (
            <div className="bg-yellow-600 px-2  rounded-md">
                {props.method}
            </div>
        )
    } else {
        return (
            <div className="bg-red-600 px-2  rounded-md">
                {props.method}
            </div>
        )
    }
}