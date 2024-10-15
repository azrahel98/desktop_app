<template>
	<div class="card  w-full !px-0">
		<header class="relative flex  flex-col font-dm  items-start justify-between px-3">
			<div class="mb-px mt-3 text-xl font-bold text-navy-700 dark:text-white">Eventos</div>
			<p class="text-start text-base text-sm text-gray-600 ">
				Eventos programados
			</p>
		</header>
		<div class="grid px-0.5 mt-5">
			<div class="evento bg-brand-900 px-2" v-for="x in eventos">
				<div class="dia bg-blueSecondary my-2">
					<span>{{ (x as any).fecha }}</span>
				</div>
				<span class="text-white w-full text-start text-sm font-medium">
					{{ (x as any).descripcion }}
				</span>
			</div>
			<div class="evento border-[1px] border-gray-200" v-for="x in cumples">
				<div class="dia my-2">
					<div class="flex flex-col">
						<cake-icon class="text-orange-600" size="25" />
						<span class="text-green-600 font-bold !text-lg">
							{{ (x as any).dia }}
						</span>
					</div>
				</div>
				<span class="w-full text-start text-xs font-medium">
					<p v-for="n in (x as any).nombres" class="py-1 flex items-center gap-1">
						<div class="dot bg-blue-500 border-blue-500 !h-1 !w-1 !border-[1px]"/>
						{{ n }}
					</p>
				</span>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	defineProps({
		eventos: { type: Array, required: true },
		cumples: { type: Array, required: true }
	})
</script>

<style lang="scss" scoped>

	.grid {
		row-gap: 1vh;
		height: 100%;
		overflow-y: auto;

		
		.evento {
			border-radius: 10px;
			display: flex;
			align-items: center;
			gap: 1vh;
			.dia {
				height: 6vh;
				color: white;
				width: 8vh;
				max-width: 8vh;
				border-radius: 10px;
				display: flex;
				justify-content: center;
				align-items: center;
				span {
					text-align: center;
					vertical-align: middle;
					font-weight: 700;
					font-size: 1.7rem;
				}
			}
		}
	}
	
</style>
