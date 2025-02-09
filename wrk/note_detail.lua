local body = [[
    {"query":"query Note($id: Int!) {\n    result: publishedNote(id: $id) {\n        id\n        title\n        content\n        type\n        images\n        video\n        updatedAt\n        views\n    }\n}","variables":{"id":2}}
]]

local headers = {
    ["Content-Type"] = "application/json",
}

function request()
    return wrk.format("POST", "/graphql", headers, body)
end
