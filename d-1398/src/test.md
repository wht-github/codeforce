**Name:** Haotian Wang
**Phone number:** +86 188-2334-4586
**E-mail:** 1479486567@qq.com

**Education**
Studying in: Southern Unversity of Science and Technology  
major: Computer Science and Technology
- GPA 3.61/4.0
- CET6 488
- Second Place Scholarship Award for Outstanding Student in the 2017 Zhicheng College
- First Prize in the 2018 Southern University of Science and Technology Programming Competition
**Experience**
[Online Judge System](https://github.com/darkliang/JudeeBE)
I was in charge of the judger and some of the backend, and mainly implemented the logic for the interaction between the judger and the backend, as well as the logic for the judging score measurement.
- Object Oriented Analysis and Design course team project (team of 5, individual contribution 25%)
- Implementation of a fully functional OJ platform
OJ platform supports Java, C/C++, Python3 and ACM/OI system. Supports batch random account generation.
database interaction using peewee (orm), measuring the code running time and memory part of the use of QDUOJ open source module. Redis is used for message queuing. The judger machine is protected against attacks by docker and ssecomp, and compilation and execution is done in docker. Support for rapid deployment of multiple judger machines.
Synchronize testcases with rsync.

[DNS proxy](https://github.com/wht-github/CS305-IP-OVER-DNS)
- Computer network course project (group of 3, individual contribution 70%)
- Simple Proxy using dns protocol
The proxy is implemented by forwarding requests with a specific query name to the personal server via authority dns.
First establish the TUN interface and modify the routing table so that the specified traffic is sent to the TUN, which allows you to modify the network layer data at the application layer.
The dnspython package is used to parse the data, and the TXT part of the dns RR is used to implement the data transfer, and the request is written in the format of a domain name, separated by '. ' separated by a '.', using base64 encoding. The half-duplex is guaranteed by heartbeat packets and avoids overload. Proxy can be done via socket5 or by modifying the routing table directly.

Course Project Teaming System
- Database system principles course project (group of 5, individual contribution 40%)
- A project teaming system for course projects based on WeChat mini-programs.
- Support for both teacher and student identity, ability to create courses, projects
- Teams support invite and direct join, and teams can be locked.
Personally responsible for the back end, did not use orm. created a DAO for some of the data.

Gomoku
- Artificial Intelligence course project
- Rely on the interface provided by the school to implement the five-player AI.
Use alpha-beta game trees and pruning, create evaluation functions raising weights for specific chess types.
