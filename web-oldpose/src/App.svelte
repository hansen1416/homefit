<script>
	import { Router, Route } from "svelte-routing";
	import Home from "./routes/Home.svelte";
</script>

<Router>
	<div>
		<!-- <Route path="/blog/:id" component={BlogPost} /> -->
		<Route path="/" component={Home} />
	</div>
</Router>
