<script>
	import { Router, Route } from "svelte-routing";
	import WebSocketClient from "./lib/WebSocketClient";
	import ExtractAnimationData from "./routes/ExtractAnimationData.svelte";
	import Scene from "./components/Scene.svelte";
	import Home from "./routes/Home.svelte";
	import Gym from "./routes/Gym.svelte";
	import _ from "lodash";

	import { websocket_state } from "./store/websocketStore";
	import animation_data from "./store/animationDataStore";

	let socket = new WebSocketClient();
	// When you write $websocket, you're essentially saying, "Get the current value from the store named websocket."

	// websocket.set(socket); // Assign the instance to the store
	// Connect to the websocket
	socket.connect("ws://localhost:3333/ws");

	socket.onConnect = (ws_state) => {
		websocket_state.set(ws_state);
	};

	socket.onMessage = (msg) => {
		// check if msg is an ArrayBuffer(0)
		if (msg instanceof ArrayBuffer && msg.byteLength === 0) {
			console.log("received heartbeat message");
			return;
		}

		if (typeof msg !== "string") {
			console.log("websocket message received non-string message", msg);
			return;
		}

		// first split the message
		let [category, name, data] = msg.split("::");

		// check if the message is for animation data
		if (category === "anim") {
			console.log("received animation data", name);

			// update animation_data
			const existing_data = $animation_data;

			existing_data[name] = data;

			animation_data.set(existing_data);
		}
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
