package org.spenblet.app

import android.content.Context
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.view.MotionEvent
import android.view.View
import kotlin.math.cos
import kotlin.math.roundToInt
import kotlin.math.sin

data class PenSample(
    val kind: String,
    val x: Int,
    val y: Int,
    val pressure: Int,
    val distance: Int,
    val tiltX: Int,
    val tiltY: Int,
    val button: Int,
) {
    fun encode() = "SPENBLET/1 $kind $x $y $pressure $distance $tiltX $tiltY $button"
}

class PenSurface(context: Context, private val server: PenServer) : View(context) {
    private val dotSpacing = 28f * resources.displayMetrics.density
    private val dotRadius = 1.15f * resources.displayMetrics.density
    private val dotPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        color = Color.rgb(52, 57, 62)
        style = Paint.Style.FILL
    }

    init { setBackgroundColor(Color.rgb(17, 19, 21)) }

    override fun onDraw(canvas: Canvas) {
        val offsetX = (width.toFloat() % dotSpacing) / 2f
        val offsetY = (height.toFloat() % dotSpacing) / 2f
        var y = offsetY
        while (y < height) {
            var x = offsetX
            while (x < width) {
                canvas.drawCircle(x, y, dotRadius, dotPaint)
                x += dotSpacing
            }
            y += dotSpacing
        }
    }

    override fun onTouchEvent(event: MotionEvent): Boolean {
        if (event.getToolType(0) != MotionEvent.TOOL_TYPE_STYLUS && event.getToolType(0) != MotionEvent.TOOL_TYPE_ERASER) return false
        return process(event)
    }

    override fun onHoverEvent(event: MotionEvent): Boolean {
        if (event.getToolType(0) != MotionEvent.TOOL_TYPE_STYLUS && event.getToolType(0) != MotionEvent.TOOL_TYPE_ERASER) return false
        return process(event)
    }

    private fun process(event: MotionEvent): Boolean {
        val kind = when (event.actionMasked) {
            MotionEvent.ACTION_DOWN -> "down"
            MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL -> "up"
            MotionEvent.ACTION_HOVER_MOVE, MotionEvent.ACTION_HOVER_ENTER, MotionEvent.ACTION_HOVER_EXIT -> "hover"
            else -> "move"
        }
        server.publish(event.toSample(kind, width, height))
        return true
    }

    private fun MotionEvent.toSample(kind: String, width: Int, height: Int): PenSample {
        fun scaled(value: Float, size: Int) = (value / size.coerceAtLeast(1) * 65535).roundToInt().coerceIn(0, 65535)
        fun axis(axis: Int, factor: Float, min: Int, max: Int) = (getAxisValue(axis) * factor).roundToInt().coerceIn(min, max)
        val button = if (buttonState and MotionEvent.BUTTON_STYLUS_PRIMARY != 0) 1 else 0
        val tilt = getAxisValue(MotionEvent.AXIS_TILT)
        val orientation = getAxisValue(MotionEvent.AXIS_ORIENTATION)
        val tiltX = (sin(orientation.toDouble()) * tilt * 9000f).roundToInt().coerceIn(-9000, 9000)
        val tiltY = (cos(orientation.toDouble()) * tilt * 9000f).roundToInt().coerceIn(-9000, 9000)
        return PenSample(kind, scaled(x, width), scaled(y, height), axis(MotionEvent.AXIS_PRESSURE, 4095f, 0, 4095), axis(MotionEvent.AXIS_DISTANCE, 255f, 0, 255), tiltX, tiltY, button)
    }
}
