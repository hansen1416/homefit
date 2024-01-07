<script>
	import { loadFBX } from "../utils/ropes";
</script>

<div class="controls">
	<!-- a select file input button -->
	<input
		type="file"
		id="file"
		name="file"
		accept=".fbx"
		on:change={(e) => {
			const file_url = URL.createObjectURL(e.target.files[0]);

			if (file_url) {
				loadFBX(file_url).then((fbx) => {
					const data = fbx.animations[0];

					console.log(data.toJSON());
				});
			}
		}}
	/>
</div>
