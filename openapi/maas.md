---
title: maas v0.1.0
language_tabs:
  - shell: Shell
  - http: HTTP
  - javascript: JavaScript
  - ruby: Ruby
  - python: Python
  - php: PHP
  - java: Java
  - go: Go
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

```http
GET /mutex/lock/{name} HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/mutex/lock/{name}',
{
  method: 'GET',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.get '/mutex/lock/{name}',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/mutex/lock/{name}', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('GET','/mutex/lock/{name}', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/mutex/lock/{name}");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("GET");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("GET", "/mutex/lock/{name}", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

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

```http
PUT /mutex/lock/{name} HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/mutex/lock/{name}',
{
  method: 'PUT',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.put '/mutex/lock/{name}',
  params: {
  }, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.put('/mutex/lock/{name}', headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('PUT','/mutex/lock/{name}', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/mutex/lock/{name}");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("PUT");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("PUT", "/mutex/lock/{name}", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`PUT /lock/{name}`

Tries to get ownership of a mutex

Tries to get ownership of a mutex with the specified `name`, waiting for a max amount of time specified in `timout`. This method will not wait longer than the maximum amount of time a mutex can be active. This is configured by the server owner.

# Arguments

* `name` - The name of the mutex.

* `timeout` - The maximum amount of seconds to wait for the mutex. Default: 60. Use 0 to return instantly.

This function returns an uuid, which is proof that you hold the lock. This uuid is needed to unlock the mutex.

<h3 id="lock-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|none|
|timeout|query|integer(uint64)|false|none|

> Example responses

> 200 Response

```json
{
  "uuid": "string",
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
curl -X DELETE /mutex/lock/{name}?uuid=string \
  -H 'Accept: application/json'

```

```http
DELETE /mutex/lock/{name}?uuid=string HTTP/1.1

Accept: application/json

```

```javascript

const headers = {
  'Accept':'application/json'
};

fetch('/mutex/lock/{name}?uuid=string',
{
  method: 'DELETE',

  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```ruby
require 'rest-client'
require 'json'

headers = {
  'Accept' => 'application/json'
}

result = RestClient.delete '/mutex/lock/{name}',
  params: {
  'uuid' => 'string'
}, headers: headers

p JSON.parse(result)

```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/mutex/lock/{name}', params={
  'uuid': 'string'
}, headers = headers)

print(r.json())

```

```php
<?php

require 'vendor/autoload.php';

$headers = array(
    'Accept' => 'application/json',
);

$client = new \GuzzleHttp\Client();

// Define array of request body.
$request_body = array();

try {
    $response = $client->request('DELETE','/mutex/lock/{name}', array(
        'headers' => $headers,
        'json' => $request_body,
       )
    );
    print_r($response->getBody()->getContents());
 }
 catch (\GuzzleHttp\Exception\BadResponseException $e) {
    // handle exception or api errors.
    print_r($e->getMessage());
 }

 // ...

```

```java
URL obj = new URL("/mutex/lock/{name}?uuid=string");
HttpURLConnection con = (HttpURLConnection) obj.openConnection();
con.setRequestMethod("DELETE");
int responseCode = con.getResponseCode();
BufferedReader in = new BufferedReader(
    new InputStreamReader(con.getInputStream()));
String inputLine;
StringBuffer response = new StringBuffer();
while ((inputLine = in.readLine()) != null) {
    response.append(inputLine);
}
in.close();
System.out.println(response.toString());

```

```go
package main

import (
       "bytes"
       "net/http"
)

func main() {

    headers := map[string][]string{
        "Accept": []string{"application/json"},
    }

    data := bytes.NewBuffer([]byte{jsonReq})
    req, err := http.NewRequest("DELETE", "/mutex/lock/{name}", data)
    req.Header = headers

    client := &http.Client{}
    resp, err := client.Do(req)
    // ...
}

```

`DELETE /lock/{name}`

Releases ownership of a mutex

Releases ownership of a mutex with the given `name`, if it is currently owned and the `uuid` matches.

# Arguments

* `name` - The name of the mutex.

* `uuid` - The maximum amount of seconds to wait for the mutex. Default: 60. Use 0 to return instantly.

returns a struct with the `name` and the `is_locked` property of a mutex.

<h3 id="unlock-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|name|path|string|true|none|
|uuid|query|string|true|none|

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
  "uuid": "string",
  "name": "string",
  "is_locked": true
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|uuid|string|true|none|none|
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

