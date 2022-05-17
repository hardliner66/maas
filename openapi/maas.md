---
title: maas v0.1.0
language_tabs:
  - shell: Shell
toc_footers: []
includes: []
search: true
highlight_theme: darkula
headingLevel: 2

---

<!-- Generator: Widdershins v4.0.1 -->

<h1 id="maas">maas v0.1.0</h1>

> Scroll down for code samples, example requests and responses. Select a language for code samples from the tabs above or the mobile navigation menu.

Base URLs:

* <a href="/mutex">/mutex</a>

<h1 id="maas-default">Default</h1>

## is_locked

<a id="opIdis_locked"></a>

> Code samples

```shell
# You can also use wget
curl -X GET /mutex/lock/{name} \
  -H 'Accept: application/json'

```

`GET /lock/{name}`

Returns if a mutex with the given name is locked

Returns a struct with the `name` and the `is_locked` property of a mutex.

# Arguments

* `name` - The name of the mutex.

<h3 id="is_locked-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|none|

> Example responses

> 200 Response

```json
{
  "name": "string",
  "is_locked": true
}
```

<h3 id="is_locked-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|[MutexData](#schemamutexdata)|

<aside class="success">
This operation does not require authentication
</aside>

## lock

<a id="opIdlock"></a>

> Code samples

```shell
# You can also use wget
curl -X PUT /mutex/lock/{name} \
  -H 'Accept: application/json'

```

`PUT /lock/{name}`

Tries to get ownership of a mutex

Tries to get ownership of a mutex with the specified `name`, waiting for a max amount of time specified in `timout`. This method will not wait longer than the maximum amount of time a mutex can be active. This is configured by the server owner.

# Arguments

* `name` - The name of the mutex.

* `timeout` - The maximum amount of seconds to wait for the mutex. Default: 60. Use 0 to return instantly.

This function returns a mutex id, which is proof that you hold the lock. This mutex id is needed to unlock the mutex.

<h3 id="lock-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|none|
|timeout|query|integer(uint64)|false|none|

> Example responses

> 200 Response

```json
{
  "mutex_id": "string",
  "name": "string",
  "is_locked": true
}
```

<h3 id="lock-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|[LockMutexData](#schemalockmutexdata)|
|default|Default|none|[CustomError](#schemacustomerror)|

<aside class="success">
This operation does not require authentication
</aside>

## unlock

<a id="opIdunlock"></a>

> Code samples

```shell
# You can also use wget
curl -X DELETE /mutex/lock/{name}?mutex_id=string \
  -H 'Accept: application/json'

```

`DELETE /lock/{name}`

Releases ownership of a mutex

Releases ownership of a mutex with the given `name`, if it is currently owned and the `mutex_id` matches.

# Arguments

* `name` - The name of the mutex.

* `mutex_id` - The maximum amount of seconds to wait for the mutex. Default: 60. Use 0 to return instantly.

returns a struct with the `name` and the `is_locked` property of a mutex.

<h3 id="unlock-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|none|
|mutex_id|query|string|true|none|

> Example responses

> 200 Response

```json
{
  "name": "string",
  "is_locked": true
}
```

<h3 id="unlock-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|[MutexData](#schemamutexdata)|
|default|Default|none|[CustomError](#schemacustomerror)|

<aside class="success">
This operation does not require authentication
</aside>

# Schemas

<h2 id="tocS_LockMutexData">LockMutexData</h2>
<!-- backwards compatibility -->
<a id="schemalockmutexdata"></a>
<a id="schema_LockMutexData"></a>
<a id="tocSlockmutexdata"></a>
<a id="tocslockmutexdata"></a>

```json
{
  "mutex_id": "string",
  "name": "string",
  "is_locked": true
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|mutex_id|string|true|none|none|
|name|string|true|none|none|
|is_locked|boolean|true|none|none|

<h2 id="tocS_CustomError">CustomError</h2>
<!-- backwards compatibility -->
<a id="schemacustomerror"></a>
<a id="schema_CustomError"></a>
<a id="tocScustomerror"></a>
<a id="tocscustomerror"></a>

```json
{
  "message": "string",
  "code": 0
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|message|string|true|none|none|
|code|integer(uint16)|true|none|none|

<h2 id="tocS_MutexData">MutexData</h2>
<!-- backwards compatibility -->
<a id="schemamutexdata"></a>
<a id="schema_MutexData"></a>
<a id="tocSmutexdata"></a>
<a id="tocsmutexdata"></a>

```json
{
  "name": "string",
  "is_locked": true
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|name|string|true|none|none|
|is_locked|boolean|true|none|none|

