import { Link, useRoutes } from "raviger"
import Home from "./pages/Home"
import RequestView from "./pages/RequestView"

const routes = {
    '/': () => <Home />,
    '/request': () => <RequestView />
}

export default function App() {
    let route = useRoutes(routes)
    return (
      <div>
        {route}
      </div>
    )
  }