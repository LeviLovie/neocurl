check_version("1.3.*")

define({
	name = "send",
	func = function()
		result = send({
			url = "https://httpbin.org/get",
			method = "GET",
			headers = {
				["User-Agent"] = "Neocurl",
				["Accept"] = "application/json",
			},
		})

		info("Response received")
		print_response(result)

		assert(result.status == 200, function()
			error("Expected status 200, got " .. result.status)
		end)
	end,
})

define({
	name = "send_status",
	func = function()
		result = send({
			url = "https://httpbin.org/post",
			method = "POST",
			headers = {
				["User-Agent"] = "Neocurl",
				["Accept"] = "application/json",
			},
		})

		info("Status: " .. result.status)
	end,
})

define({
	name = "send_async",
	func = function()
		result = send_async({
			url = "https://httpbin.org/post",
			method = "POST",
			headers = {
				["User-Agent"] = "Neocurl",
				["Accept"] = "application/json",
			},
		}, 25)

		print(dump(result))
	end,
})

define({
	name = "send_async_stress",
	test = false,
	func = function()
		send_async({
			url = "https://httpbin.org/post",
			method = "POST",
			headers = {
				["User-Agent"] = "Neocurl",
				["Accept"] = "application/json",
			},
		}, 5000, 200)
	end,
})

define({
	name = "time",
	func = function()
		info(time())
		info(format_time("%Y-%m-%d %H:%M:%S"))
	end,
})

define({
	name = "load_download",
	func = function()
		info(load("./src/ncurl.rs"))
		info(download("https://raw.githubusercontent.com/LeviLovie/neocurl/refs/heads/main/src/neocurl.rs"))
	end,
})

define({
	name = "env",
	func = function()
		info(env("HOME"))
	end,
})

define({
	name = "pass",
	func = function()
		info("Passing...")
	end,
})

define({
	name = "fail",
	test = false,
	func = function()
		assert(false, function()
			error("This is a failure test")
		end)
	end,
})

define({
	name = "run",
	func = function()
		run("send")
	end,
})

define({
	name = "many",
	func = function()
		run("send_status", 5, false)
	end,
})

define({
	name = "many_async",
	func = function()
		run_async({ "send_status" }, 5)
	end,
})

define({
	name = "async",
	func = function()
		run_async({ "send", "send_status" }, 25, false)
	end,
})

define({
	name = "stress_async",
	test = false,
	func = function()
		run_async({ "send", "send_status" }, 500, true, 40)
	end,
})

define({
	name = "base64",
	func = function()
		payload = "Hello, World!"
		encoded = to_base64(payload)
		decoded = from_base64(encoded)

		info("Payload: " .. payload .. ", encoded: " .. encoded .. ", decoded: " .. decoded)

		assert(payload == decoded, function()
			error("Base64 decode failed: expected '" .. payload .. "', got '" .. decoded .. "'")
		end)
	end,
})

define({
	name = "json",
	func = function()
		json = require("json")
		local tbl = {
			animals = { "dog", "cat", "aardvark" },
			instruments = { "violin", "trombone", "theremin" },
			bugs = json.null,
			trees = nil,
		}

		local str = json.encode(tbl, { indent = false })

		local obj, pos, err = json.decode(str, 1, nil)
		if err then
			error("Error:", err)
		end

		info(dump(tbl))
		info("JSON: " .. str)
		info(dump(obj))

		assert(obj.animals[1] == "dog", function()
			error("Expected 'dog', got '" .. obj.animals[1] .. "'")
		end)
		assert(obj.instruments[2] == "trombone")
	end,
})

define({
	name = "send",
	func = function()
		send({
			url = "https://httpbin.org/get",
			method = "GET",
			headers = {
				["User-Agent"] = "Neocurl",
				["Accept"] = "application/json",
			},
		})
	end,
})

define({
	name = "logs",
	func = function()
		debug("Debug message")
		info("Info message")
		warn("Warning message")
		error("Error message")
	end,
})

define({
	name = "async_logs",
	func = function()
		run_async({ "logs" }, 10)
	end,
})
