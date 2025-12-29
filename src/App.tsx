import { Router, Route } from "@solidjs/router";
import { Header } from "./components/Header";
import { TaskPage } from "./pages/TaskPage";
import { CompletedPage } from "./pages/CompletedPage";
import { ArchivedPage } from "./pages/ArchivedPage";
import { TagManagementPage } from "./pages/TagManagementPage";
import type { RouteSectionProps } from "@solidjs/router";

// Root Layout Component - Router context内で動作
function RootLayout(props: RouteSectionProps) {
  return (
    <div class="flex h-screen flex-col bg-background rounded-xl overflow-hidden">
      <Header />
      <div class="flex-1 overflow-hidden">
        {props.children}
      </div>
    </div>
  );
}

function App() {
  return (
    <Router root={RootLayout}>
      <Route path="/" component={TaskPage} />
      <Route path="/completed" component={CompletedPage} />
      <Route path="/archive" component={ArchivedPage} />
      <Route path="/tags" component={TagManagementPage} />
    </Router>
  );
}

export default App;
