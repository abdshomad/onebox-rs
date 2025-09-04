import React from 'react';
import { Deck, Slide, Heading, ListItem, List, Text, Image, Appear } from 'spectacle';

const theme = {
  colors: {
    primary: 'white',
    secondary: '#1F2022',
    tertiary: '#03A9FC',
    quaternary: '#CECECE',
  },
  fonts: {
    header: '"Yanone Kaffeesatz", Helvetica, sans-serif',
    body: '"Droid Serif", "Helvetica Neue", Helvetica, sans-serif',
  },
};

const Presentation = () => (
  <Deck theme={theme}>
    <Slide>
      <Heading>onebox-rs</Heading>
      <Text>High-performance, secure internet bonding</Text>
      <Text>A presentation by Jules</Text>
    </Slide>
    <Slide>
      <Heading>The Problem</Heading>
      <List>
        <ListItem>Low Bandwidth</ListItem>
        <ListItem>Poor Reliability</ListItem>
        <ListItem>Lack of Redundancy</ListItem>
        <ListItem>High Cost of dedicated lines</ListItem>
      </List>
    </Slide>
    <Slide>
      <Heading>The Solution: onebox-rs</Heading>
      <Text>A Rust-based internet bonding solution that aggregates multiple internet connections into a single, resilient virtual connection.</Text>
    </Slide>
    <Slide>
      <Heading>Features</Heading>
      <Appear>
        <Text>Internet Bonding: Combine multiple WAN connections for increased bandwidth.</Text>
        <Text>Seamless Failover: Automatic failover when connections drop, with zero packet loss.</Text>
        <Text>End-to-End Encryption: ChaCha20-Poly1305 encryption for all tunnel traffic.</Text>
        <Text>High Performance: Built with Rust and Tokio for minimal CPU overhead.</Text>
      </Appear>
    </Slide>
    <Slide>
      <Heading>Architecture</Heading>
    </Slide>
    <Slide>
      <Heading>Client (`onebox-client`)</Heading>
      <List>
        <ListItem>Runs on Linux-based single-board computers (e.g., Raspberry Pi)</ListItem>
        <ListItem>Creates a TUN interface to capture all outgoing traffic</ListItem>
        <ListItem>Distributes packets across multiple connections</ListItem>
      </List>
    </Slide>
    <Slide>
      <Heading>Server (`onebox-server`)</Heading>
      <List>
        <ListItem>Runs on a cloud VPS with a public IP</ListItem>
        <ListItem>Receives encrypted packets from clients</ListItem>
        <ListItem>Reassembles packets and forwards them to the internet</ListItem>
      </List>
    </Slide>
    <Slide>
      <Image src="https://i.imgur.com/9y7B42s.png" width="100%" />
    </Slide>
    <Slide>
      <Heading>Technical Details</Heading>
      <List>
        <ListItem>Language: Rust</ListItem>
        <ListItem>Async Runtime: Tokio</ListItem>
        <ListItem>Protocol: Custom UDP-based protocol</ListItem>
        <ListItem>Encryption: ChaCha20-Poly1305</ListItem>
        <ListItem>Configuration: TOML</ListItem>
      </List>
    </Slide>
    <Slide>
      <Heading>Future Work</Heading>
      <Appear>
        <Text>Web Dashboard: A simple web interface for real-time monitoring and configuration.</Text>
        <Text>Advanced Bonding Modes: Implement different policies, such as "reliability first" or "cost-aware".</Text>
        <Text>WireGuard Integration: Explore using WireGuard as the underlying transport.</Text>
        <Text>Cross-Platform Client: Expand client support to other operating systems like macOS and Windows.</Text>
      </Appear>
    </Slide>
    <Slide>
      <Heading>The End</Heading>
      <Text>Created with Spectacle</Text>
    </Slide>
  </Deck>
);

export default Presentation;
