import { Link, useRoutes } from "raviger"
import Home from "./pages/Home"
import RequestView from "./pages/RequestView"
import { QueryClientProvider, QueryClient } from '@tanstack/react-query'

const queryClient = new QueryClient();


const routes = {
  '/': () => <Home />,
  '/request': () => <RequestView />
}

export default function App() {
  let route = useRoutes(routes)
  return (
    <QueryClientProvider client={queryClient}>
      <div>
        {route}
      </div>
    </QueryClientProvider>

  )
}
