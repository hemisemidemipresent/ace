import { SIZE, R, TAU, DOT_SIZE } from './utils/Constants.jsx';
import { onMount } from "solid-js";

export default () => {
    let canvas;
    onMount(() => {
        const ctx = canvas.getContext("2d");

        // dark background
        // ctx.fillStyle = "#001234";
        // ctx.fillRect(0, 0, canvas.width, canvas.height);

        // draw center point
        ctx.fillStyle = "#ffffff";
        ctx.fillRect(SIZE/2, SIZE/2, 1, 1)

        ctx.strokeStyle = "#ffffff";
        // ctx.lineWidth = DOT_SIZE / 8;
        // draw circle
        ctx.beginPath();
        ctx.arc(SIZE/2, SIZE/2, R, 0, TAU);
        ctx.stroke();
        // draw 8
        ctx.beginPath();
        ctx.arc(SIZE/2, SIZE/2 - R/2, R/2, 0, TAU);
        ctx.stroke();
        ctx.beginPath();
        ctx.arc(SIZE/2, SIZE/2 + R/2, R/2, 0, TAU);
        ctx.stroke();
        // draw infinity
        ctx.beginPath();
        ctx.arc(SIZE/2 - R/2, SIZE/2, R/2, 0, TAU);
        ctx.stroke();
        ctx.beginPath();
        ctx.arc(SIZE/2 + R/2, SIZE/2, R/2, 0, TAU);
        ctx.stroke();
    })
    return <canvas ref={canvas} width={SIZE} height={SIZE} />
}