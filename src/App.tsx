import { Link, useRoutes } from "raviger"
import Home from "./pages/Home"
import RequestView from "./pages/RequestView"
import { QueryClientProvider, QueryClient } from '@tanstack/react-query'
import RequestEditPage from "./pages/RequestEdit";

const queryClient = new QueryClient();


const routes = {
  '/': () => <Home />,
  '/:requestId/request': () => <RequestView />,
  '/:requestId/edit': () => <RequestEditPage />
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
