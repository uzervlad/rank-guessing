<script lang="ts">
    import { browser } from "$app/environment";

  export const ssr = false;

  import { onDestroy, onMount } from "svelte";

  let { auto }: { auto: boolean } = $props();

  type Particle = {
    x: number;
    y: number;
    vx: number;
    vy: number;
    size: number;
    color: string;
    life: number;
  };

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  const onResize = () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
  };

  const COLORS = ["#ff4d4d", "#4dff88", "#4da6ff", "#ffd24d", "#ff66cc"];

  let particles: Particle[] = [];
  let side = 0;
  let animationId: number;
  let burstTimeout: number | null;
  let lastMs = 0;

  function scheduleNextBurst() {
    if (!auto) return;
    burstTimeout = setTimeout(() => {
      spawnBurst();
    }, 800 + Math.random() * 300);
  }

  export const spawnBurst = () => {
    const count = 60 + Math.random() * 10;
    const fromLeft = side === 0;

    for (let i = 0; i < count; i++) {
      particles.push({
        x: fromLeft ? -20 : canvas.width + 20,
        y: Math.random() * canvas.height * 0.6 + canvas.height * 0.2,
        vx: (fromLeft ? 1 : -1) * (350 + Math.random() * 850),
        vy: (Math.random() - 0.55) * 2000,
        size: 4 + Math.random() * 3,
        color: COLORS[Math.floor(Math.random() * COLORS.length)],
        life: 3,
      });
    }

    side = 1 - side;

    if (burstTimeout) clearTimeout(burstTimeout);
    scheduleNextBurst();
  };

  $effect(() => {
    if (burstTimeout) {
      clearTimeout(burstTimeout);
      burstTimeout = null;
    }
    if (auto) scheduleNextBurst();
  });

  const render = (ms: number) => {
    let delta = (ms - lastMs) / 1000;
    lastMs = ms;

    animationId = requestAnimationFrame(render);

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (let i = particles.length - 1; i >= 0; i--) {
      let p = particles[i];

      p.x += p.vx * delta;
      p.y += p.vy * delta;
      p.vy += 1700 * delta;
      p.life -= delta;

      ctx.fillStyle = p.color;
      ctx.beginPath();
      ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
      ctx.fill();

      if (p.life <= 0) particles.splice(i, 1);
    }
  };

  onMount(() => {
    ctx = canvas.getContext("2d")!;
    if (browser) {
      onResize();
      window.addEventListener("resize", onResize);
      render(0);
    }
  });

  onDestroy(() => {
    if (browser) {
      window.removeEventListener("resize", onResize);
      cancelAnimationFrame(animationId);
    }
  });
</script>

<canvas bind:this={canvas}></canvas>

<style lang="scss">
  canvas {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    pointer-events: none;
  }
</style>