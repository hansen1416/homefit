<script>
	import { Router, Route } from "svelte-routing";
	import { websocket, websocket_state } from "./store/websocketStore";
	import WebSocketClient from "./lib/WebSocketClient";
	import ExtractAnimationData from "./routes/ExtractAnimationData.svelte";
	import Scene from "./components/Scene.svelte";
	import Home from "./routes/Home.svelte";
	import Gym from "./routes/Gym.svelte";

	let socket = new WebSocketClient();
	// When you write $websocket, you're essentially saying, "Get the current value from the store named websocket."
	$websocket = socket; // Assign the instance to the store

	// Connect to the websocket
	socket.connect("ws://localhost:3333/ws");

	socket.onConnect = (state) => {
		$websocket_state = state;
	};
</script>

<Router>
	<div>
		<Scene />
		<Route path="/" component={Home} />
		<Route path="/extract" component={ExtractAnimationData} />
		<Route path="/gym/:id" let:params>
			<Gym id={params.id} />
		</Route>
	</div>
</Router>
