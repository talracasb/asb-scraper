<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="centered">
	<h1 class="day">{data.day}</h1>

	<h2 class="welcome">Welcome {data.student_name}</h2>
</div>

<h1>betterasb</h1>
<ul class="links">
	<li>
		<a href="/grades">Assignment Grades</a>
	</li>
	<li>
		<a href="/schedule">Schedule</a>
	</li>
</ul>

<h1>myasb</h1>
<ul class="links">
	<li>
		<a href="https://sites.google.com/asbarcelona.com/lantern">Lantern</a>
	</li>
	<li>
		<a href="https://docs.google.com/document/d/1zc67zXQVrimob0CYOS5wj_lPj6roDDfM2mSrPFTSo98/edit"
			>Morning Announcements</a
		>
	</li>
	<li>
		<a href="https://myasb.asbarcelona.com/s/my_shorteners/">Link Generator</a>
	</li>
	<li>
		<a href="https://printing.asbarcelona.com:9192/">Web Print</a>
	</li>
</ul>

<style lang="less">
	.centered {
		width: 100%;
		display: flex;
		align-items: center;
		flex-direction: column;
		gap: 1rem;
		margin-top: 3rem;
		margin-bottom: 3rem;
	}

	.day {
		margin-top: 0;
		margin-bottom: 0;
		font-size: 3rem;
		text-align: center;
	}

	.welcome {
		margin-top: 0;
		margin-bottom: 0;
	}
</style>
