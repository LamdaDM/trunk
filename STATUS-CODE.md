# Codetables

Messages are prepended with two bytes for the response's status code.

The first byte determines if there is a response or error. If there is
an error, the code determines whether the issue is internal or with
the client's request.

The second byte is for determining what the issue 
relates to with no-response statuses.

## **Primary**

Code | Response | Error | Description
-----|----------|-------|------------------------
00   |Yes       |No     | OK
10   |No        |Client | Bad request
20   |No        |Server | Internal error
30   |No        |No     | Cannot serve

## **Secondary**

Code | Issue
-----|------------------
0    | N/A
1    | Service-related
2    | Dependency-related
