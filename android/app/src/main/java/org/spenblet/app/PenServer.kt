package org.spenblet.app

import android.util.Log
import java.io.Closeable
import java.io.OutputStreamWriter
import java.net.ServerSocket
import java.net.Socket
import java.util.concurrent.ArrayBlockingQueue
import java.util.concurrent.CopyOnWriteArrayList
import kotlin.concurrent.thread

class PenServer : Closeable {
    private class Client(private val socket: Socket, private val onClosed: (Client) -> Unit) : Closeable {
        private val queue = ArrayBlockingQueue<String>(256)
        private val worker = thread(name = "spenblet-client", isDaemon = true) {
            try {
                OutputStreamWriter(socket.getOutputStream(), Charsets.UTF_8).buffered().use { writer ->
                    while (!socket.isClosed) {
                        writer.write(queue.take())
                        writer.newLine()
                        writer.flush()
                    }
                }
            } catch (error: Exception) {
                if (!socket.isClosed) Log.w(TAG, "Client disconnected", error)
            } finally {
                try { socket.close() } catch (_: Exception) { }
                onClosed(this)
            }
        }

        fun publish(line: String) {
            if (!queue.offer(line)) {
                queue.poll()
                queue.offer(line)
            }
        }

        override fun close() {
            try { socket.close() } catch (_: Exception) { }
            worker.interrupt()
        }
    }

    private val clients = CopyOnWriteArrayList<Client>()
    private var socket: ServerSocket? = null

    fun start() {
        thread(name = "spenblet-server", isDaemon = true) {
            try {
                ServerSocket(PORT).use { server ->
                    socket = server
                    while (!server.isClosed) accept(server)
                }
            } catch (error: Exception) {
                Log.e(TAG, "Server stopped", error)
            } finally {
                socket = null
            }
        }
    }

    private fun accept(server: ServerSocket) {
        val socket = server.accept()
        socket.tcpNoDelay = true
        val client = Client(socket) { clients -= it }
        clients += client
        Log.i(TAG, "Client connected: ${clients.size}")
    }

    fun publish(sample: PenSample) {
        val line = sample.encode()
        clients.forEach { it.publish(line) }
    }

    override fun close() {
        try { socket?.close() } catch (_: Exception) { }
        clients.forEach { it.close() }
        clients.clear()
    }

    private companion object {
        const val PORT = 27183
        const val TAG = "spenblet"
    }
}
