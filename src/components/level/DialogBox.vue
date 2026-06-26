<script setup lang="ts">
import { useSlots, ref, watch, onMounted, onUnmounted, computed } from 'vue';

defineSlots<{
    name(): any
    lastName(): any
    faction(): any
    profession(): any
    dialogText(): any
    DialogMenu(): any
    cursor(): any
}>();

type DialogBoxStyleVariables = {
    dividerColor: string;
    nameColor: string;
    lastNameColor: string;
    factionColor: string;
    professionColor: string;
    dialogColor: string;
    backColor: string;
    boxShadow: string;
    nameFontSize: string;
    lastNameFontSize: string;
    factionFontSize: string;
    professionFontSize: string;
    dialogFontSize: string;
    topFontSize: string;
};

export interface Props {
    coordinates?: "left" | "right" | "top" | "bottom";
    textCoordinates?: "left" | "right" | "top" | "bottom";
    menuCoordinates?: "topLeftCorner" | "topRightCorner" | "bottomLeftCorner" | "bottomRightCorner";
    styleVariables?: DialogBoxStyleVariables;
    speed?: number;
    trigger?: number;
    visible?: boolean;
};

const props = withDefaults(defineProps<Props>(), {
    coordinates: "bottom",
    menuCoordinates: "bottomRightCorner",
    textCoordinates: "left",
    styleVariables: () => {
        return {
            dividerColor: "#ffffff",
            nameColor: "#69a6ec",
            lastNameColor: "#69a6ec",
            factionColor: "#69a6ec",
            professionColor: "#69a6ec",
            dialogColor: "#f0f8ff",
            backColor: "#c0c0c015",
            boxShadow: " 0 0 15px #ffffff",
            nameFontSize: "1.5rem",
            lastNameFontSize: "1.5rem",
            factionFontSize: "1rem",
            professionFontSize: "1rem",
            dialogFontSize: "1rem",
            topFontSize: "1rem",
        };
    },
    speed: 50,
    trigger: 0,
    visible: true
});

type Segment =
    | { type: 'text', char: string }
    | { type: 'tag', tag: string, isClosing: boolean };

const slots = useSlots();
const displayHtml = ref('');
let segments: Segment[] = [];
let currentIndex = 0;
let timer: number | null = null;

function vnodeToSegments(vnodes: any[]): Segment[] {
    const result: Segment[] = [];
    function traverse(node: any) {
        if (typeof node === 'string' || typeof node === 'number') {
            const text = String(node);
            for (const ch of text) {
                result.push({ type: 'text', char: ch })
            };
            return;
        };
        if (node && typeof node === 'object' && node.type) {
            if (typeof node.type === 'string') {
                const tagName = node.type;
                result.push({ type: 'tag', tag: tagName, isClosing: false });

                let children = node.children;
                if (children) {
                    if (!Array.isArray(children)) children = [children];
                    for (const child of children) {
                        traverse(child);
                    };
                };
                result.push({ type: 'tag', tag: tagName, isClosing: true });
            } else {
                if (node.children) {
                    let children = node.children;
                    if (!Array.isArray(children)) children = [children]
                    for (const child of children) {
                        traverse(child);
                    };
                };
            };
        };
    };

    for (const vnode of vnodes) {
        traverse(vnode);
    };
    return result;
}

function startTyping() {
    if (timer) {
        clearInterval(timer);
        timer = null;
    }

    displayHtml.value = '';
    currentIndex = 0;

    const slotContent = slots.dialogText ? slots.dialogText() : [];
    segments = vnodeToSegments(slotContent);

    if (segments.length === 0) return;

    timer = window.setInterval(() => {
        if (currentIndex >= segments.length) {
            clearInterval(timer!);
            timer = null;
            return;
        };

        const seg = segments[currentIndex];
        if (!seg) {
            currentIndex++;
            return;
        };
        if (seg.type === 'text') {
            displayHtml.value += seg.char;
        } else {
            displayHtml.value += seg.isClosing ? `</${seg.tag}>` : `<${seg.tag}>`;
        }
        currentIndex++;
    }, props.speed);
};

watch(() => props.trigger, () => {
    startTyping();
});

onMounted(() => {
    startTyping();
});

onUnmounted(() => {
    if (timer) clearInterval(timer);
});

defineExpose({
    restart: startTyping
});

const cssVars = computed(() => {
    const positionMap = {
        left: {
            "--topTop": 0,
            "--topLeft": 0,
            "--topBottom": 0,
            "--topRight": "auto",
            "--topWidth": "30%",
            "--topHeight": "100%"
        },
        right: {
            "--topTop": 0,
            "--topRight": 0,
            "--topBottom": 0,
            "--topLeft": "auto",
            "--topWidth": "30%",
            "--topHeight": "100%"
        },
        top: {
            "--topTop": 0,
            "--topLeft": 0,
            "--topRight": 0,
            "--topBottom": "auto",
            "--topWidth": "100%",
            "--topHeight": "30%"
        },
        bottom: {
            "--topBottom": 0,
            "--topLeft": 0,
            "--topRight": 0,
            "--topTop": "auto",
            "--topWidth": "100%",
            "--topHeight": "30%"
        }
    };
    const ret = {
        "--dividerColor": props.styleVariables.dividerColor,
        "--nameColor": props.styleVariables.nameColor,
        "--lastNameColor": props.styleVariables.lastNameColor,
        "--factionColor": props.styleVariables.factionColor,
        "--professionColor": props.styleVariables.professionColor,
        "--dialogColor": props.styleVariables.dialogColor,
        "--backColor": props.styleVariables.backColor,
        "--boxShadow": props.styleVariables.boxShadow,
        "--nameFontSize": props.styleVariables.nameFontSize,
        "--lastNameFontSize": props.styleVariables.lastNameFontSize,
        "--factionFontSize": props.styleVariables.factionFontSize,
        "--professionFontSize": props.styleVariables.professionFontSize,
        "--dialogFontSize": props.styleVariables.dialogFontSize,
        "--topFontSize": props.styleVariables.topFontSize,
        ...positionMap[props.coordinates]
    };
    return ret;
})

const menuVars = computed(() => {
    const basePos = {
        topLeftCorner: {
            "--topTop": 0,
            "--topLeft": 0,
            "--topBottom": "auto",
            "--topRight": "auto",
        },
        topRightCorner: {
            "--topTop": 0,
            "--topRight": 0,
            "--topBottom": "auto",
            "--topLeft": "auto",
        },
        bottomLeftCorner: {
            "--topTop": "auto",
            "--topLeft": 0,
            "--topRight": "auto",
            "--topBottom": 0,
        },
        bottomRightCorner: {
            "--topTop": "auto",
            "--topLeft": "auto",
            "--topRight": 0,
            "--topBottom": 0,
        }
    };
    return {
        ...basePos[props.menuCoordinates],
    };
});

</script>

<template>
    <div class="MainBox" :style="cssVars" v-show="props.visible">
        <div class="container">
            <div class="char">
                <div class="names">
                    <span class="name">
                        <slot name="name"></slot>
                    </span>
                    <span class="lastName">
                        <slot name="lastName"></slot>
                    </span>
                </div>
                <div class="factions">
                    <span class="faction">
                        <slot name="faction"></slot>
                    </span>
                </div>
                <div class="professions">
                    <span class="profession">
                        <slot name="profession"></slot>
                    </span>
                </div>
            </div>
            <p class="divider"></p>
            <div class="dialog">
                <span class="text" v-html="displayHtml"></span>
                <slot name="cursor"></slot>
            </div>
        </div>
    </div>
    <div class="menuWrapper" :style="menuVars">
        <slot name="DialogMenu"></slot>
    </div>
</template>

<style scoped lang="scss">
.MainBox {
    font-size: var(--topFontSize);
    position: fixed;
    bottom: var(--topBottom);
    left: var(--topLeft);
    right: var(--topRight);
    top: var(--topTop);
    width: var(--topWidth);
    height: var(--topHeight);
    max-height: 100vh;
    max-width: 100vw;
}

.container {
    padding: 0.5rem;
    position: relative;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: var(--backColor);
    box-shadow: var(--boxShadow);
    display: flex;
    flex-direction: column;

    .char {
        flex: 0 0 15%;
        display: flex;
        align-items: flex-end;
        gap: 0.8rem;
        height: 15%;

        .names {
            display: flex;
            gap: 0.5rem;

            .name {
                font-size: var(--nameFontSize);
                color: var(--nameColor);
            }

            .lastName {
                font-size: var(--lastNameFontSize);
                color: var(--lastNameColor);
            }
        }

        .faction {
            font-size: var(--factionFontSize);
            color: var(--factionColor);
        }

        .profession {
            font-size: var(--professionFontSize);
            color: var(--professionColor);
        }
    }

    .divider {
        flex: 0 0 0.1%;
        width: 100%;
        background-color: var(--dividerColor);
    }

    .dialog {
        flex: 0 0 84%;
        font-size: var(--dialogFontSize);
        color: var(--dialogColor);
    }
}

.menuWrapper {
    position: fixed;
    bottom: var(--topBottom);
    left: var(--topLeft);
    right: var(--topRight);
    top: var(--topTop);
    display: flex;
    justify-content: flex-end;
    align-items: flex-end;
    pointer-events: auto;
    z-index: 10;
    padding: 0.5rem;
}
</style>