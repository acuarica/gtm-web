
import { WebApp } from './web';

class WebAppProxy {

  constructor(opts: {
    target: Element;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    props?: Record<string, any> | undefined;
  }) {
    new WebApp(window.location.search, opts.target)
  }

}

export default [{
  component: WebAppProxy,
  name: 'WebApp',
  props: {
  },
}]