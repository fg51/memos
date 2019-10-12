
## setup

```sh
$ vue create <app name>
* typescript: Yes
* class method: Yes

$ vue add tsx-support
```

## name
* data -> class filed
* computed -> get, set
* methods -> instance method
* watch -> @Watch
* props -> @Prop
* render -> render(h: CreateElement): VNode

## sample

```typescript
- import <xyz> from HelloWorld.vue;
+ import <xyz> from HelloWorld;
```

```typescript
import { Component, Prop, Vue } from "vue-property-decorator";
import { CreateElement, VNode } from "vue";

@Component
export default class HelloWorld extends Vue {

  isMember: boolean = true;

  member = {
    name: "foo"
  }

  @Prop({ default: "" })
  msg!: string;

  render(h: CreateElement): VNode {
    return (
      <div>
        <h1>{this.msg}</h1>
        {this.isMember ? (
          <h2>Hi {this.member.name}</h2>
        ) : (
          <h2>Hi No Name User</h2>
        )}
      </div>
    );
  }
}
```
