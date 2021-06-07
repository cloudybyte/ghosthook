# ghosthook

## What is ghosthook?
I created ghosthook to serve as an adapter between the Ghost blogging platform and various other platforms (first and foremost Discord). <br>
Feel free to open a issue or PR to add support for more.

## Setup
It's pretty straight forward: Either download the precompiled binary from the [releases page](https://github.com/cloudybyte/ghosthook/releases) or get the sourcecode and compile it yourself.<br><br>
1.  Create a <code>config.toml</code> with all the config parameters shown below.<br>
2. Create a new custom ghost integration the the admin dashboard and to point the url to <code>http://<host_uri>/post/published</code>.
3. Try to start the adapter and publish a test post to ensure everything is working as indented. If any errors occur, please open an issue.


### Configuration 
ghosthook uses a config file in the toml format. For the required and optional parameters, please see below.

<table>
<thead>
<tr>
<th>config value</th>
<th>use</th>
<th>example</th>
<th>default</th>
<th>required?</th>
</tr>
</thead>
<tbody>
<tr>
<td><code>host_uri</code></td>
<td>Set the ip and port to bind to</td>
<td><code>127.0.01:2354</code></td>
<td align="center">-/-</td>
<td align="center">yes</td>
</tr>
<tr>
<td><code>preview_length</code></td>
<td>Set the amount of text that will be previewed<br>in the hook's message</td>
<td><code>150</code></td>
<td align="center">-/-</td>
<td align="center">yes</td>
</tr>
</tr>
<tr>
<td><code>webhook_username</code></td>
<td>The name the app uses in its webhooks</td>
<td><code>ghosthook</code></td>
<td align="center">-/-</td>
<td align="center">yes</td>
</tr>
</tr>
<tr>
<td><code>webhook_avatar_url</code></td>
<td>Set the avatar the webhooks uses</td>
<td><code>https://example.com/ghosthook.png</code></td>
<td align="center">-/-</td>
<td align="center">yes</td>
</tr>
</tr>
<tr>
<td><code>discord_webhooks</code></td>
<td>This can hold an unlimited amount of credentials to discord webhooks</td>
<td><code>[{id = 123, token = "tkn"}, {id = 321, token = "TOKEN"}]</code></td>
<td align="center">-/-</td>
<td align="center">yes</td>
</tr>
</tbody>
</table>

### Available endpoints

<table>
<thead>
<tr>
<th>Endpoint</th>
<th>Use</th>
</tr>
</thead>
<tbody>
<tr>
<td><code>/post/published</code></td>
<td>Trigger the <code>Post Published</code> webhook message</td>
</tr>

</tbody>
</table>


## Legal
This project is licensed under either the [Apache License, Version 2.0](https://choosealicense.com/licenses/apache-2.0/) or [MIT](https://choosealicense.com/licenses/mit/) license at your option.<br>
Unless you explicitly state otherwise, any contribution submitted by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 