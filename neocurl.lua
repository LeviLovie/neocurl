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

        assert("status", result.status == 200)
        assert_not("status", result.status > 400)
        assert_eq("status", result.status, 200)
        assert_ne("status", result.status, 404)

        assert("status", result.status ~= 200)
        assert_not("status", result.status < 400)
        assert_eq("status", result.status, 400)
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

        print_response(result)
    end,
})

