<script lang="ts">
    let { animating = false } = $props();

    // https://codepen.io/dtsprague/pen/YOjjwg
    const pathString = path(9, 0.2, 13, 15, 0.5);

    function sin(x: number) {
        return trunc(Math.sin(2 * Math.PI * x / 360));
    }

    function cos(x: number) {
        return trunc(Math.cos(2 * Math.PI * x / 360));
    }

    function trunc(x: number) {
        return Number(x.toFixed(3).substring(0, 5));
    }

    function path(toothCount: number, depth: number, toothAngle: number, rootAngle: number, holeRadius: number) {
        const pitchAngle = 360 / toothCount;
        const rampAngle = (pitchAngle - toothAngle - rootAngle) / 2;
        const r = 1 - depth;
        let text = "M0,-1";
        let angle = 0;

        for (let i = 0; i < toothCount; i++) {
            angle += toothAngle / 2;
            text += "A1,1,0,0,1," + sin(angle) + "," + -cos(angle);
            angle += rampAngle;
            text += "L" + trunc(r * sin(angle)) + "," + trunc(-r * cos(angle));
            angle += rootAngle;
            text +=
            "A" +
            trunc(r) +
            "," +
            trunc(r) +
            ",0,0,1," +
            trunc(r * sin(angle)) +
            "," +
            trunc(-r * cos(angle));
            angle += rampAngle;
            text += "L" + sin(angle) + "," + -cos(angle);
            angle += toothAngle / 2;
            text += "A1,1,0,0,1," + sin(angle) + "," + -cos(angle);
        }

        text += `L0,${-holeRadius}A${holeRadius},${holeRadius},0,1,0,0,${holeRadius}A${holeRadius},${holeRadius},0,1,0,0,${-holeRadius}Z`;

        return text;
    }
</script>

<svg viewBox="-1.1, -1.1, 2.2, 2.2" fill="currentColor" width="14px" class="gear" class:animating={animating}>
    <path d={pathString}>
    </path>
</svg>

<style lang="scss">
    .gear {
        animation: rotate 2s linear infinite;
        animation-play-state: paused;
    }

    .animating {
        animation-play-state: running;
    }

    @-webkit-keyframes rotate {
        from { -webkit-transform: rotate(0) }
        to { -webkit-transform: rotate(359deg) }
    }

    @keyframes rotate {
        from { transform: rotate(0) }
        to { transform: rotate(359deg) }
    }
</style>
