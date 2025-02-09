local body = [[
    {"query":"query publishedNotes {\n    publishedNotes {\n        id\n        views\n    }\n}","variables":{}}
]]

local headers = {
    ["Content-Type"] = "application/json",
}

function request()
    return wrk.format("POST", "/graphql", headers, body_json)
end
