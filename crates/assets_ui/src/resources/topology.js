// UAVRed Network Topology - D3.js Visualization
// This script expects: nodes, links, zones, nodeScaleFactor to be defined

(function() {
    'use strict';

    // State
    let svg, g, zoom, simulation;
    let link, node, label, labelBg;
    let selectedNode = null;
    let width, height, zoneWidth;
    let zoneNodeCounts = {};

    // Zone configuration
    const headerHeight = 55;
    const MAX_HOVER_LINKS = 20;
    
    // Global variables for element access
    let baseRadius = 6;
    let connectionCounts = {};

    // Initialize when DOM is ready
    function init() {
        width = window.innerWidth;
        height = window.innerHeight;
        zoneWidth = width / zones.length;

        // Count nodes per zone
        zones.forEach((z, i) => {
            zoneNodeCounts[i] = nodes.filter(n => n.zoneIndex === i).length;
        });

        createSVG();
        createDefs();
        createZoneBackgrounds();
        createSimulation();
        createElements();
        setupInteractions();
        setupZoom();
        setupResizeHandler();

        // Apply initial transform after simulation stabilizes
        setTimeout(() => {
            const t = calculateOptimalTransform();
            zoom.scaleExtent([t.k * 0.9, 3]);
            svg.call(zoom.transform, d3.zoomIdentity.translate(t.x, t.y).scale(t.k));
        }, 600);
    }

    function createSVG() {
        svg = d3.select('#graph')
            .append('svg')
            .attr('width', width)
            .attr('height', height);

        g = svg.append('g');
    }

    function createDefs() {
        const defs = svg.append('defs');

        // Arrow marker for normal links
        defs.append('marker')
            .attr('id', 'arrow-normal')
            .attr('viewBox', '0 -5 10 10')
            .attr('refX', 15)
            .attr('refY', 0)
            .attr('markerWidth', 4)
            .attr('markerHeight', 4)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M0,-4L8,0L0,4')
            .attr('fill', '#9ca3af');

        // Arrow marker for highlighted links
        defs.append('marker')
            .attr('id', 'arrow-highlighted')
            .attr('viewBox', '0 -5 10 10')
            .attr('refX', 15)
            .attr('refY', 0)
            .attr('markerWidth', 5)
            .attr('markerHeight', 5)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M0,-4L8,0L0,4')
            .attr('fill', '#4b5563');

        // Arrow marker for cross-zone links
        defs.append('marker')
            .attr('id', 'arrow-crosszone')
            .attr('viewBox', '0 -5 10 10')
            .attr('refX', 15)
            .attr('refY', 0)
            .attr('markerWidth', 4)
            .attr('markerHeight', 4)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M0,-4L8,0L0,4')
            .attr('fill', '#6b7280');
    }

    function createZoneBackgrounds() {
        const zoneGroups = g.selectAll('.zone-group')
            .data(zones)
            .join('g')
            .attr('class', 'zone-group');

        // Zone background rectangles
        zoneGroups.append('rect')
            .attr('class', 'zone-bg')
            .attr('x', (d, i) => i * zoneWidth)
            .attr('y', 0)
            .attr('width', zoneWidth)
            .attr('height', height)
            .attr('fill', d => d[2])
            .attr('fill-opacity', 0.5);

        // Zone separators
        zoneGroups.filter((d, i) => i > 0)
            .append('line')
            .attr('x1', (d, i) => (i + 1) * zoneWidth)
            .attr('y1', 0)
            .attr('x2', (d, i) => (i + 1) * zoneWidth)
            .attr('y2', height)
            .attr('stroke', '#d1d5db')
            .attr('stroke-width', 1);

        // Zone headers
        const headerGroups = zoneGroups.append('g').attr('class', 'zone-header');

        headerGroups.append('circle')
            .attr('cx', (d, i) => i * zoneWidth + 16)
            .attr('cy', 20)
            .attr('r', 8)
            .attr('fill', d => d[3]);

        headerGroups.append('text')
            .attr('class', 'zone-label')
            .attr('x', (d, i) => i * zoneWidth + 30)
            .attr('y', 16)
            .attr('fill', d => d[3])
            .text(d => d[0]);

        headerGroups.append('text')
            .attr('class', 'zone-sublabel')
            .attr('x', (d, i) => i * zoneWidth + 30)
            .attr('y', 30)
            .text(d => d[1]);

        headerGroups.append('text')
            .attr('class', 'zone-count')
            .attr('x', (d, i) => i * zoneWidth + 30)
            .attr('y', 42)
            .text((d, i) => zoneNodeCounts[i] + ' 资产');
    }

    function createSimulation() {
        const zoneCenters = zones.map((_, i) => ({
            x: i * zoneWidth + zoneWidth / 2,
            y: (height + headerHeight) / 2
        }));

        const baseRadius = Math.max(4, 6 * nodeScaleFactor);
        const densityFactor = Math.min(1, 200 / totalAssets);

        simulation = d3.forceSimulation(nodes)
            .alphaDecay(0.02)
            .velocityDecay(0.3)
            .force('link', d3.forceLink(links)
                .id(d => d.id)
                .distance(d => 40 * densityFactor + 20))
            .force('charge', d3.forceManyBody()
                .strength(d => -50 * densityFactor * nodeScaleFactor)
                .distanceMin(10)
                .distanceMax(300))
            .force('collision', d3.forceCollide()
                .radius(d => baseRadius + 6)
                .strength(1.0)
                .iterations(3))
            .force('x', d3.forceX(d => zoneCenters[d.zoneIndex].x).strength(0.5 * densityFactor))
            .force('y', d3.forceY(d => zoneCenters[d.zoneIndex].y).strength(0.2))
            .force('boundary', createBoundaryForce(baseRadius));
    }

    function createBoundaryForce(baseRadius) {
        const minPadding = baseRadius + 10;
        return () => {
            nodes.forEach(d => {
                const r = baseRadius + 5;
                const zoneLeft = d.zoneIndex * zoneWidth + minPadding;
                const zoneRight = (d.zoneIndex + 1) * zoneWidth - minPadding;
                const zoneTop = headerHeight + minPadding;
                const zoneBottom = height - minPadding;

                const k = 0.1;
                if (d.x < zoneLeft + r) d.vx += (zoneLeft + r - d.x) * k;
                if (d.x > zoneRight - r) d.vx -= (d.x - (zoneRight - r)) * k;
                if (d.y < zoneTop + r) d.vy += (zoneTop + r - d.y) * k;
                if (d.y > zoneBottom - r) d.vy -= (d.y - (zoneBottom - r)) * k;

                d.x = Math.max(zoneLeft + r, Math.min(zoneRight - r, d.x));
                d.y = Math.max(zoneTop + r, Math.min(zoneBottom - r, d.y));
            });
        };
    }

    function createElements() {
        // Calculate connection counts (store in global variable)
        connectionCounts = {};
        links.forEach(l => {
            const s = l.source.id || l.source;
            const t = l.target.id || l.target;
            connectionCounts[s] = (connectionCounts[s] || 0) + 1;
            connectionCounts[t] = (connectionCounts[t] || 0) + 1;
        });

        // Links
        link = g.append('g')
            .attr('class', 'links')
            .selectAll('line')
            .data(links)
            .join('line')
            .attr('class', d => {
                const sourceNode = nodes.find(n => n.id === (d.source.id || d.source));
                const targetNode = nodes.find(n => n.id === (d.target.id || d.target));
                const isCrossZone = sourceNode && targetNode && sourceNode.zoneIndex !== targetNode.zoneIndex;
                return isCrossZone ? 'link cross-zone' : 'link';
            })
            .attr('marker-end', d => {
                const sourceNode = nodes.find(n => n.id === (d.source.id || d.source));
                const targetNode = nodes.find(n => n.id === (d.target.id || d.target));
                const isCrossZone = sourceNode && targetNode && sourceNode.zoneIndex !== targetNode.zoneIndex;
                return isCrossZone ? 'url(#arrow-crosszone)' : 'url(#arrow-normal)';
            });

        // Nodes
        baseRadius = Math.max(4, 6 * nodeScaleFactor);
        node = g.append('g')
            .attr('class', 'nodes')
            .selectAll('circle')
            .data(nodes)
            .join('circle')
            .attr('class', d => 'node' + (d.vulnCount > 0 ? ' vulnerable' : ''))
            .attr('r', d => {
                const connections = connectionCounts[d.id] || 0;
                const hubBonus = connections > 5 ? 2 : connections > 2 ? 1 : 0;
                return baseRadius + hubBonus;
            })
            .attr('fill', d => d.color);

        // Labels
        const labelGroup = g.append('g').attr('class', 'labels');

        labelBg = labelGroup.selectAll('rect')
            .data(nodes)
            .join('rect')
            .attr('class', 'label-bg')
            .attr('x', d => -40)
            .attr('y', d => baseRadius + 2)
            .attr('width', 80)
            .attr('height', 12)
            .attr('opacity', 0);

        label = labelGroup.selectAll('text')
            .data(nodes)
            .join('text')
            .attr('class', 'label')
            .text(d => d.name.length > 15 ? d.name.substring(0, 12) + '...' : d.name)
            .attr('text-anchor', 'middle')
            .attr('y', d => baseRadius + 11);

        // Drag behavior
        node.call(d3.drag()
            .on('start', dragstarted)
            .on('drag', dragged)
            .on('end', dragended));
    }

    function setupInteractions() {
        const tooltip = d3.select('#tooltip');

        node.on('click', function(event, d) {
            event.stopPropagation();
            d3.selectAll('.node').classed('selected', false);
            d3.select(this).classed('selected', true);
            selectedNode = d.id;
            updateSelection(d);
            if (window.ipc) {
                window.ipc.postMessage('SELECT:' + d.id);
            }
        });

        node.on('dblclick', function(event, d) {
            event.stopPropagation();
            const transform = d3.zoomIdentity
                .translate(width / 2, height / 2)
                .scale(1.8)
                .translate(-d.x, -d.y);
            svg.transition().duration(500).call(zoom.transform, transform);
        });

        node.on('mouseover', function(event, d) {
            d3.select(this.parentNode).selectAll('.label')
                .filter(n => n.id === d.id)
                .classed('visible', true);

            const outgoingCount = links.filter(l => (l.source.id || l.source) === d.id).length;
            const incomingCount = links.filter(l => (l.target.id || l.target) === d.id).length;
            const totalConnections = outgoingCount + incomingCount;
            const isSuperConnector = totalConnections > MAX_HOVER_LINKS;

            const connectedLinks = links.map((l, idx) => {
                const sourceId = l.source.id || l.source;
                const targetId = l.target.id || l.target;
                const isConnected = (sourceId === d.id || targetId === d.id);
                const isOutgoing = (sourceId === d.id);
                const isCrossZone = l.source.zoneIndex !== l.target.zoneIndex;
                return { link: l, idx, isConnected, isOutgoing, isCrossZone };
            }).filter(x => x.isConnected);

            connectedLinks.sort((a, b) => (b.isCrossZone - a.isCrossZone));

            const linksToHighlight = isSuperConnector ? connectedLinks.slice(0, MAX_HOVER_LINKS) : connectedLinks;
            const highlightIndices = new Set(linksToHighlight.map(x => x.idx));

            link.each(function(l, idx) {
                const connInfo = connectedLinks.find(c => c.idx === idx);
                const isHighlighted = highlightIndices.has(idx);
                const isConnected = connInfo !== undefined;
                const isOutgoing = connInfo ? connInfo.isOutgoing : false;

                d3.select(this)
                    .style('stroke-opacity', isHighlighted ? 0.9 : (isConnected ? 0.1 : 0.03))
                    .style('stroke-width', isHighlighted ? 2.5 : (isConnected ? 1 : 0.3))
                    .style('stroke', isHighlighted ? (isOutgoing ? '#2563eb' : '#059669') : null)
                    .attr('marker-end', isHighlighted ?
                        (isOutgoing ? 'url(#arrow-highlighted)' : 'url(#arrow-crosszone)') :
                        (l.source.zoneIndex !== l.target.zoneIndex ? 'url(#arrow-crosszone)' : 'url(#arrow-normal)'));
            });

            const connectedNodeIds = new Set();
            linksToHighlight.forEach(l => {
                connectedNodeIds.add(l.link.source.id || l.link.source);
                connectedNodeIds.add(l.link.target.id || l.link.target);
            });

            node.style('opacity', n => {
                if (n.id === d.id) return 1;
                if (connectedNodeIds.has(n.id)) return 0.9;
                return isSuperConnector ? 0.2 : 0.3;
            });

            const riskClass = d.risk >= 70 || d.vulnCount > 0 ? 'risk-high' :
                              d.risk >= 40 ? 'risk-medium' : 'risk-low';

            let tooltipHtml = `<strong>${d.name}</strong>`;
            tooltipHtml += `<span class="${riskClass}">风险: ${d.risk}/100 ${d.vulnCount > 0 ? '(' + d.vulnCount + '漏洞)' : ''}</span><br>`;
            tooltipHtml += `IP: ${d.ip || 'N/A'} | 区域: ${d.group}<br>`;

            if (outgoingCount > 0) {
                tooltipHtml += `<span style="color:#2563eb">→ 可访问 ${outgoingCount} 个资产</span>`;
                if (isSuperConnector) {
                    tooltipHtml += ` <span style="color:#6b7280;font-size:10px">(显示前${MAX_HOVER_LINKS}个)</span>`;
                }
                tooltipHtml += '<br>';
            }

            if (incomingCount > 0) {
                tooltipHtml += `<span style="color:#059669">← 被 ${incomingCount} 个资产访问</span><br>`;
            }

            if (d.vulnCount > 0) {
                tooltipHtml += '<span style="color:#ef4444">⚠ 存在漏洞</span>';
            }

            tooltip.html(tooltipHtml)
                .style('left', (event.pageX + 12) + 'px')
                .style('top', (event.pageY - 12) + 'px')
                .classed('show', true);
        });

        node.on('mouseout', function(event, d) {
            if (selectedNode !== d.id) {
                label.filter(n => n.id === d.id).classed('visible', false);
            }

            link.each(function(l) {
                const isCrossZone = l.source.zoneIndex !== l.target.zoneIndex;
                d3.select(this)
                    .style('stroke-opacity', null)
                    .style('stroke-width', null)
                    .style('stroke', null)
                    .attr('marker-end', isCrossZone ? 'url(#arrow-crosszone)' : 'url(#arrow-normal)');
            });

            node.style('opacity', null);
            tooltip.classed('show', false);
        });

        svg.on('click', () => {
            d3.selectAll('.node').classed('selected', false);
            selectedNode = null;
            label.classed('visible', false);
            labelBg.attr('opacity', 0);
            link.style('stroke-opacity', null).style('stroke-width', null);
        });

        simulation.on('tick', () => {
            link
                .attr('x1', d => d.source.x)
                .attr('y1', d => d.source.y)
                .attr('x2', d => d.target.x)
                .attr('y2', d => d.target.y);

            node
                .attr('cx', d => d.x)
                .attr('cy', d => d.y);

            const baseRadius = Math.max(4, 6 * nodeScaleFactor);
            labelBg
                .attr('x', d => d.x - 40)
                .attr('y', d => d.y + baseRadius + 1);

            label
                .attr('x', d => d.x)
                .attr('y', d => d.y + baseRadius + 11);
        });
    }

    function setupZoom() {
        const contentWidth = width;
        const contentHeight = height;
        const initialScale = Math.min(
            width / contentWidth * 0.95,
            height / contentHeight * 0.95,
            1
        );

        zoom = d3.zoom()
            .scaleExtent([initialScale * 0.8, 3])
            .on('zoom', (event) => {
                g.attr('transform', event.transform);
            });

        svg.call(zoom);
    }

    function setupResizeHandler() {
        let resizeTimeout;
        window.addEventListener('resize', () => {
            clearTimeout(resizeTimeout);
            resizeTimeout = setTimeout(handleResize, 250);
        });
    }

    function handleResize() {
        const newWidth = window.innerWidth;
        const newHeight = window.innerHeight;

        width = newWidth;
        height = newHeight;
        zoneWidth = width / zones.length;

        svg.attr('width', width).attr('height', height);

        // Update zone backgrounds
        g.selectAll('.zone-group rect')
            .attr('width', zoneWidth)
            .attr('height', height)
            .attr('x', (d, i) => i * zoneWidth);

        // Update zone separators
        g.selectAll('.zone-group line')
            .attr('x1', (d, i) => (i + 1) * zoneWidth)
            .attr('x2', (d, i) => (i + 1) * zoneWidth)
            .attr('y2', height);

        // Update zone headers
        g.selectAll('.zone-header circle')
            .attr('cx', (d, i) => i * zoneWidth + 16);

        g.selectAll('.zone-header text')
            .attr('x', (d, i) => i * zoneWidth + 30);

        // Update forces with new zone centers
        const zoneCenters = zones.map((_, i) => ({
            x: i * zoneWidth + zoneWidth / 2,
            y: (height + headerHeight) / 2
        }));

        simulation.force('x', d3.forceX(d => zoneCenters[d.zoneIndex].y).strength(0.5));
        simulation.alpha(0.3).restart();

        // Recalculate optimal zoom
        setTimeout(() => {
            const t = calculateOptimalTransform();
            zoom.scaleExtent([t.k * 0.9, 3]);
            svg.transition().duration(300).call(
                zoom.transform,
                d3.zoomIdentity.translate(t.x, t.y).scale(t.k)
            );
        }, 300);
    }

    function updateSelection(d) {
        label.classed('visible', n => n.id === d.id || n.id === selectedNode);
        labelBg.attr('opacity', n => n.id === d.id || n.id === selectedNode ? 1 : 0);
    }

    function dragstarted(event, d) {
        if (!event.active) simulation.alphaTarget(0.3).restart();
        d.fx = d.x;
        d.fy = d.y;
    }

    function dragged(event, d) {
        d.fx = event.x;
        d.fy = event.y;
    }

    function dragended(event, d) {
        if (!event.active) simulation.alphaTarget(0);
        d.fx = null;
        d.fy = null;
    }

    function calculateOptimalTransform() {
        const bounds = g.node().getBBox();
        const paddingX = 10;
        const paddingY = 10;
        const availableWidth = width - paddingX * 2;
        const availableHeight = height - paddingY * 2;
        const scaleX = availableWidth / bounds.width;
        const scaleY = availableHeight / bounds.height;
        const scale = Math.min(scaleX, scaleY);

        return {
            x: paddingX - bounds.x * scale,
            y: paddingY - bounds.y * scale,
            k: scale
        };
    }

    // Exposed control functions
    window.zoomIn = function() {
        svg.transition().call(zoom.scaleBy, 1.3);
    };

    window.zoomOut = function() {
        svg.transition().call(zoom.scaleBy, 0.7);
    };

    window.resetZoom = function() {
        const t = calculateOptimalTransform();
        svg.transition().duration(500).call(
            zoom.transform,
            d3.zoomIdentity.translate(t.x, t.y).scale(t.k)
        );
    };

    // API for external calls
    window.updateTopologyData = function(newNodes, newLinks) {
        console.log('Updating topology data:', newNodes.length, 'nodes');
    };

    window.highlightNode = function(nodeId) {
        d3.selectAll('.node').classed('selected', d => d.id === nodeId);
        label.classed('visible', d => d.id === nodeId);
    };

    // Focus and zoom to a specific node
    window.focusNode = function(nodeId) {
        const targetNode = nodes.find(n => n.id === nodeId);
        if (!targetNode) {
            console.warn('Node not found:', nodeId);
            return;
        }
        
        // Highlight the node
        d3.selectAll('.node').classed('selected', d => d.id === nodeId);
        selectedNode = nodeId;
        label.classed('visible', d => d.id === nodeId);
        labelBg.attr('opacity', d => d.id === nodeId ? 1 : 0);
        
        // Calculate zoom transform to center on the node
        // Use a moderate zoom level (1.5x) for better context
        const focusScale = 1.5;
        const x = width / 2 - targetNode.x * focusScale;
        const y = height / 2 - targetNode.y * focusScale;
        
        // Animate to the node position
        svg.transition()
            .duration(750)
            .ease(d3.easeCubicOut)
            .call(
                zoom.transform,
                d3.zoomIdentity.translate(x, y).scale(focusScale)
            );
        
        // Flash the node to draw attention
        const nodeSelection = node.filter(d => d.id === nodeId);
        const originalRadius = baseRadius + ((connectionCounts[nodeId] || 0) > 5 ? 2 : (connectionCounts[nodeId] || 0) > 2 ? 1 : 0);
        nodeSelection
            .transition()
            .duration(200)
            .attr('r', originalRadius * 1.5)
            .transition()
            .duration(200)
            .attr('r', originalRadius);
        
        console.log('Focused on node:', nodeId, 'at position:', targetNode.x, targetNode.y);
    };

    // Initialize
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
