import { SIZE, R, TAU } from './utils/Constants.jsx';
import { createEffect } from "solid-js";

function circle(ctx, x, y, r, squishfactor) {
    y *= squishfactor
    x += SIZE / 2
    y += SIZE / 2
    ctx.beginPath();
    ctx.ellipse(x, y, r, r * squishfactor, 0, 0, TAU);
    ctx.stroke();
}

export default (props) => {
    let canvas;

    function draw() {
        const ctx = canvas.getContext("2d");
        ctx.strokeStyle = "#ffffff";

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // draw circle
        circle(ctx, 0, 0, R, props.squishfactor)

        // draw 8
        circle(ctx, 0, R/2, R/2, props.squishfactor)
        circle(ctx, 0, -R/2, R/2, props.squishfactor)

        // draw infinity
        circle(ctx, R/2, 0, R/2, props.squishfactor)
        circle(ctx, -R/2, 0, R/2, props.squishfactor)
    }

    // onMount(() => {
    //     draw()
    // })
    // createEffect includes onMount
    createEffect(() => draw())
    return <canvas ref={canvas} width={SIZE} height={SIZE} id={props.squishfactor}/>
}