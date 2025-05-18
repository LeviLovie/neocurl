request({
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
        assert_not("status < 400", result.status > 400)
        assert_eq("status", result.status, 200)
        assert_ne("status", result.status, 404)

        assert("200 status", result.status ~= 200)
        assert_not("status < 400", result.status < 400)
        assert_eq("status == 400", result.status, 400)
        assert_ne("status", result.status, 200)
    end,
})

request({
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

request({
    name = "test_run",
    func = function()
        run("test2")
    end,
})

request({
    name = "test_many",
    func = function()
        run("test2", 5)
    end,
})
