<?hh

function init() {
  require __DIR__ . '/../../util/server_tests.inc';
  ServerUtilServerTests::$LOG_ROOT = '/tmp/hhvm_server';
  require __DIR__ . '/../Adoy/Client.php';

  ServerUtilServerTests::$DOC_ROOT = dirname(__DIR__).'/server_root';
  ServerUtilServerTests::$request = fastcgi_request<>;
}

function runTest($testController, $customArgs = '') {
  $pid = posix_getpid();
  try {
    $serverProc = $serverPort = $adminPort = null;
    $debugPort = false;
    $serverProc = startServer(inout $serverPort, inout $adminPort, inout $debugPort,
                              dirname(__DIR__), ServerUtilServerTests::$DOC_ROOT, $customArgs);
    $testController($serverPort);
    stopServer($adminPort, $serverProc);
  } catch (Exception $e) {
    error_log("Caught exception, test failed, pid=$pid, exn=".$e->getMessage());
    killChildren($pid);
    if ($serverProc) proc_close($serverProc);
    error_log('test failed');
  }
}

function fastcgi_request($host, $port, $path, $post = darray[], $headers = darray[], $extra_args = darray[]) {
  try {
    $client = new \Adoy\FastCGI\Client($host, $port);
    $content = http_build_query($post);

    $args = array_merge(darray[
      'GATEWAY_INTERFACE' => 'FastCGI/1.0',
      'REQUEST_METHOD' => (!($post ?? false)) ? 'GET' : 'POST',
      'SERVER_SOFTWARE' => 'php/fcgiclient',
      'SERVER_PROTOCOL' => 'HTTP/1.1',
      'CONTENT_LENGTH' => strlen($content)
    ], $extra_args);
    foreach($headers as $k => $v) {
      $args["HTTP_" . strtoupper($k)] = $v;
    }
    $pos = strpos($path, '?');
    if ($pos !== false) {
      $args['QUERY_STRING'] = substr($path, $pos + 1);
      $path = substr($path, 0, $pos);
    }
    $doc_root = ServerUtilServerTests::$DOC_ROOT;
    $pos = strpos($path, '//');
    if ($pos) {
      $doc_root .= '/' . substr($path, 0, $pos);
      $path = substr($path, $pos + 2);
    }
    $args['SCRIPT_FILENAME'] = $doc_root . '/' . $path;
    $args['SCRIPT_NAME'] = $path;
    $args['DOCUMENT_ROOT'] = $doc_root;

    @$result = $client->request($args, $content);

    $pos = strpos($result, "\r\n\r\n");
    if ($pos !== false) {
      $result = substr($result, $pos + 4);
    }
    return trim($result);
  } catch (Exception $e) {
    tlog("request exception: " . $e->getMessage() .'('.$e->getTraceAsString().')');
    return "";
  }
}
