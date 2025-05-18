define({
    name = "test",
    func = function()
        result = send({
            url = "https://httpbin.org/get",
            method = "GET",
            headers = {
                ["User-Agent"] = "Neocurl",
                ["Accept"] = "application/json"
            },
        })

        print_response(result)

        assert("200 status", result.status == 200)
        assert_not("status >= 400", result.status >= 400)
        assert_eq("status == 200", result.status, 200)
        assert_ne("status != 404", result.status, 404)

        -- assert("200 status", result.status ~= 200)
        -- assert_not("status < 400", result.status < 400)
        -- assert_eq("status == 400", result.status, 400)
        -- assert_ne("status", result.status, 200)
    end,
})

define({
    name = "test2",
    func = function()
        result = send({
            url = "https://httpbin.org/post",
            method = "POST",
            headers = {
                ["User-Agent"] = "Neocurl",
                ["Accept"] = "application/json"
            },
        })

        print("Status: " .. result.status)
    end,
})

define({
    name = "pass",
    func = function()
        print("Passing...")
    end,
})

define({
    name = "fail",
    func = function()
        assert("Failing", false)
    end,
})

define({
    name = "test_run",
    func = function()
        run("test2")
    end,
})

define({
    name = "test_many",
    func = function()
        run("test2", 5)
    end,
})

define({
    name = "test_many_async",
    func = function()
        run_async({"test2"}, 5)
    end,
})

define({
    name = "test_async",
    func = function()
        run_async({"test2", "pass"}, 25)
    end,
})

define({
    name = "stress_async",
    func = function()
        run_async({"test", "test2"}, 500, 25)
    end,
})
